//! # Boveda Core — Command Facade
//!
//! Este módulo expone **todos** los comandos de la aplicación como métodos de
//! [`AppState`]. Es framework-agnostico: no depende de Tauri ni de ninguna UI.
//!
//! La capa Tauri (u otra UI) actúa como delegador delgado:
//! ```rust,ignore
//! #[tauri::command]
//! pub async fn unlock_vault(password: SecretString, state: State<'_, AppState>) -> Result<String, String> {
//!     state.cmd_unlock_vault(password).await
//! }
//! ```

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::auth::TotpSetupPayload;
use crate::crypto;
use crate::crypto::secret::SecretString;
use crate::storage;
use crate::vault::{BovedaEngine, ImportStrategy};
use uuid::Uuid;

// ─── AppState ─────────────────────────────────────────────────────────────────

/// Estado global compartido entre todos los manejadores de comandos.
#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Mutex<Option<BovedaEngine>>>,
    pub session_verified: Arc<Mutex<bool>>,
    pub db_path: PathBuf,
}

impl AppState {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            engine: Arc::new(Mutex::new(None)),
            session_verified: Arc::new(Mutex::new(false)),
            db_path,
        }
    }

    /// Obtiene el engine o devuelve error si el baúl está bloqueado o la sesión no está verificada.
    /// SEC-H4: Safely handle mutex poisoning instead of panicking.
    fn get_engine(&self) -> Result<BovedaEngine, String> {
        let session_ok = *self.session_verified.lock().unwrap_or_else(|e| e.into_inner());
        if !session_ok {
            return Err("Sesión no verificada. Se requiere autenticación TOTP.".to_string());
        }
        let lock = self.engine.lock()
            .map_err(|e| format!("Vault lock poisoned: {}. Por favor, reinicia la aplicación.", e))?;
        lock.as_ref().cloned().ok_or_else(|| "El baúl está bloqueado".to_string())
    }

    /// Obtiene el engine sin requerir verificación de sesión (solo para procesos de auth TOTP)
    fn get_engine_unverified(&self) -> Result<BovedaEngine, String> {
        let lock = self.engine.lock()
            .map_err(|e| format!("Vault lock poisoned: {}. Por favor, reinicia la aplicación.", e))?;
        lock.as_ref().cloned().ok_or_else(|| "El baúl está bloqueado".to_string())
    }

    // =========================================================================
    // 🔒 VAULT LIFECYCLE
    // =========================================================================

    pub fn cmd_is_vault_initialized(&self) -> bool {
        BovedaEngine::is_initialized(&self.db_path)
    }

    /// Desbloquea el baúl. Devuelve `"totp_required"` o `"unlocked"`.
    /// SEC-H3: Implements rate limiting to prevent brute force attacks on vault unlock.
    pub async fn cmd_unlock_vault(&self, password: SecretString) -> Result<String, String> {
        // SEC-H3: Rate limiting - prevent brute force unlock attempts.
        // SEC-H5: Use app_data_dir() instead of a relative path so the lock file is always
        // in a deterministic, OS-appropriate location regardless of the process CWD.
        let lock_file = Self::app_data_dir().join(".vault_unlock_lock");
        let failed_unlock_str = std::fs::read_to_string(&lock_file)
            .ok()
            .and_then(|content| content.lines().next().map(|s| s.to_string()))
            .unwrap_or_else(|| "0:0".to_string());
        
        let parts: Vec<&str> = failed_unlock_str.split(':').collect();
        let failed_attempts = parts.first().and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let last_fail_ts = parts.get(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        
        if failed_attempts >= 10 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            
            if now - last_fail_ts < 3600 { // 1 hour cooldown
                return Err(format!(
                    "Demasiados intentos fallidos de desbloqueo. Intenta en {} segundos.",
                    3600 - (now - last_fail_ts)
                ));
            }
        }
        
        let engine = match BovedaEngine::unlock(&self.db_path, &password).await {
            Ok(e) => {
                // Reset failed attempts on successful unlock
                let _ = std::fs::remove_file(&lock_file);
                e
            }
            Err(e) => {
                // Increment failed attempts
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs() as i64;
                let new_attempts = (failed_attempts + 1).to_string();
                let _ = std::fs::write(&lock_file, format!("{}:{}", new_attempts, now));
                
                // Log failed attempt
                eprintln!("[SECURITY] Intento fallido de desbloqueo del baúl: {}", e);
                
                return Err(e.to_string());
            }
        };

        let is_totp = engine.is_totp_enabled().await.unwrap_or(false);

        let mut engine_lock = self.engine.lock()
            .map_err(|e| format!("Vault lock poisoned: {}", e))?;
        *engine_lock = Some(engine);

        *self.session_verified.lock().unwrap() = !is_totp;

        if is_totp {
            Ok("totp_required".to_string())
        } else {
            Ok("unlocked".to_string())
        }
    }

    pub fn cmd_lock_vault(&self) {
        if let Ok(mut lock) = self.engine.lock() {
            if let Some(engine) = lock.as_ref() {
                engine.lock();
            }
            *lock = None;
        } else {
            eprintln!("[WARNING] Failed to acquire lock when locking vault. The engine may be in an inconsistent state.");
        }
        if let Ok(mut session) = self.session_verified.lock() {
            *session = false;
        }
    }

    /// Elimina completamente el baúl (archivos .bvda y .salt) si la contraseña es correcta.
    pub async fn cmd_delete_vault(&self, password: SecretString) -> Result<(), String> {
        // 1. Verify password by attempting unlock (this handles rate limiting automatically)
        let engine = BovedaEngine::unlock(&self.db_path, &password).await.map_err(|e| e.to_string())?;
        
        // 2. Explicitly close the connection
        engine.close().await;

        // 3. Clear the global lock to release any handles
        self.cmd_lock_vault();

        // 4. Delete files
        let salt_path = self.db_path.with_file_name("vault.salt");
        let _ = std::fs::remove_file(&self.db_path);
        let _ = std::fs::remove_file(&salt_path);

        Ok(())
    }

    // =========================================================================
    // 📁 ACCOUNT MANAGEMENT
    // =========================================================================

    pub async fn cmd_add_account(
        &self,
        site: SecretString,
        username: SecretString,
        password: SecretString,
        recovery_code: SecretString,
        notes: SecretString,
    ) -> Result<String, String> {
        let engine = self.get_engine()?;
        let recovery_opt = if recovery_code.as_str().is_empty() { None } else { Some(recovery_code) };
        let notes_opt = if notes.as_str().is_empty() { None } else { Some(notes) };
        engine
            .add_account(site, username, password, recovery_opt, notes_opt)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_get_accounts(&self) -> Result<Vec<crate::storage::models::Account>, String> {
        let engine = self.get_engine()?;
        engine.get_accounts().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_account(&self, id: &str) -> Result<(), String> {
        // SEC-C2: Validate UUID to prevent invalid IDs from being used
        Uuid::parse_str(id)
            .map_err(|_| format!("ID de cuenta inválido: '{}'. Debe ser un UUID válido.", id))?;
        
        let engine = self.get_engine()?;
        engine.delete_account(id).await.map_err(|e| e.to_string())
    }

    // =========================================================================
    // 🔒 PIN MANAGEMENT
    // =========================================================================

    pub async fn cmd_add_pin(
        &self,
        name: SecretString,
        pin: SecretString,
        notes: SecretString,
    ) -> Result<String, String> {
        let engine = self.get_engine()?;
        let notes_opt = if notes.as_str().is_empty() { None } else { Some(notes) };
        engine
            .add_pin(name, pin, notes_opt)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_get_pins(&self) -> Result<Vec<crate::storage::models::Pin>, String> {
        let engine = self.get_engine()?;
        // SEC: Log access to decrypted pin values
        let _ = engine.log_audit(crate::audit::AuditAction::SecretAccess, Some("pin_values")).await;
        engine.get_pins().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_pin(&self, id: &str) -> Result<(), String> {
        // SEC-C2: Validate UUID to prevent invalid IDs from being used
        Uuid::parse_str(id)
            .map_err(|_| format!("ID de PIN inválido: '{}'. Debe ser un UUID válido.", id))?;
        
        let engine = self.get_engine()?;
        engine.delete_pin(id).await.map_err(|e| e.to_string())
    }

    /// Genera una contraseña aleatoria (nunca se almacena).
    pub fn cmd_generate_password(length: usize, use_symbols: bool) -> Result<String, String> {
        let len = length.clamp(8, 128);
        crypto::generate_password(len, use_symbols)
            .map(|s| s.as_str().to_string())
            .map_err(|e| e.to_string())
    }

    /// Descifra un campo secreto individual bajo demanda.
    pub async fn cmd_decrypt_secret(&self, ciphertext: &str) -> Result<String, String> {
        let engine = self.get_engine()?;
        
        // SOC2: Logueamos el acceso al secreto
        let _ = engine.log_audit(crate::audit::AuditAction::SecretAccess, Some(ciphertext)).await;

        engine
            .decrypt_secret(ciphertext)
            .map(|s: SecretString| s.as_str().to_string())
            .map_err(|e: crate::BovedaError| e.to_string())
    }

    // =========================================================================
    // 👥 GROUP MANAGEMENT
    // =========================================================================

    pub async fn cmd_update_account_group(
        &self,
        id: &str,
        group_name: Option<&str>,
    ) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine
            .update_account_group(id, group_name)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_rename_group(&self, old_name: &str, new_name: &str) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine
            .rename_group(old_name, new_name)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_group(&self, name: &str) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine.delete_group(name).await.map_err(|e| e.to_string())
    }

    // =========================================================================
    // ⚙️  PREFERENCES
    // =========================================================================

    pub async fn cmd_get_preference(&self, key: &str) -> Result<Option<String>, String> {
        let engine = self.get_engine()?;
        engine
            .get_preference(key)
            .await
            .map_err(|e: crate::BovedaError| e.to_string())
    }

    pub async fn cmd_set_preference(&self, key: &str, value: Option<String>) -> Result<(), String> {
        let engine = self.get_engine()?;
        match value {
            Some(v) => engine
                .set_preference(key, &v)
                .await
                .map_err(|e: crate::BovedaError| e.to_string()),
            None => engine
                .delete_preference(key)
                .await
                .map_err(|e: crate::BovedaError| e.to_string()),
        }
    }

    // =========================================================================
    // 🖼️  BACKGROUND IMAGE
    // =========================================================================

    /// Copia una imagen al directorio de datos de la app como `background.<ext>`.
    pub async fn cmd_import_background_image(src_path: &str) -> Result<String, String> {
        let src = std::path::Path::new(src_path);
        if !src.exists() {
            return Err("El archivo de imagen no existe.".to_string());
        }

        let ext = src
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg")
            .to_lowercase();

        let allowed = ["jpg", "jpeg", "png", "webp", "gif"];
        if !allowed.contains(&ext.as_str()) {
            return Err("Formato de imagen no soportado. Usa JPG, PNG, WEBP o GIF.".to_string());
        }

        let dest_dir = Self::app_data_dir();
        for old_ext in &allowed {
            let _ = std::fs::remove_file(dest_dir.join(format!("background.{}", old_ext)));
        }

        let dest_filename = format!("background.{}", ext);
        let dest = dest_dir.join(&dest_filename);
        std::fs::copy(src_path, &dest)
            .map_err(|e| format!("Error al copiar imagen: {}", e))?;

        Ok(dest_filename)
    }

    /// Lee una imagen de fondo y la devuelve como data URL (evita exponer rutas al frontend).
    pub fn cmd_get_background_data_url(filename: &str) -> Result<String, String> {
        let path = Self::app_data_dir().join(filename);

        let bytes = std::fs::read(&path)
            .map_err(|e| format!("Cannot read background image: {}", e))?;

        let ext = std::path::Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpeg")
            .to_lowercase();

        let mime = match ext.as_str() {
            "png"  => "image/png",
            "gif"  => "image/gif",
            "webp" => "image/webp",
            _      => "image/jpeg",
        };

        use base64::Engine as _;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
        Ok(format!("data:{};base64,{}", mime, encoded))
    }

    // =========================================================================
    // 💾 FILESYSTEM UTILITIES
    // =========================================================================

    /// Devuelve la ruta del directorio de datos de la app.
    pub fn cmd_get_data_dir() -> String {
        Self::app_data_dir().to_string_lossy().to_string()
    }

    /// Exporta la base de datos y su salt a la ruta indicada.
    pub async fn cmd_export_db(dest_path: &str) -> Result<(), String> {
        let db_path = Self::vault_db_path();
        let salt_path = db_path.with_file_name("vault.salt");

        let dest_db = std::path::Path::new(dest_path);
        let dest_salt = dest_db.with_extension(format!(
            "{}.salt",
            dest_db.extension().and_then(|e| e.to_str()).unwrap_or("bvda")
        ));

        std::fs::copy(&db_path, dest_db).map_err(|e| e.to_string())?;
        if salt_path.exists() {
            std::fs::copy(&salt_path, &dest_salt).map_err(|e| e.to_string())?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(dest_db, std::fs::Permissions::from_mode(0o600));
            if dest_salt.exists() {
                let _ = std::fs::set_permissions(&dest_salt, std::fs::Permissions::from_mode(0o600));
            }
        }

        Ok(())
    }

    /// Exporta el baúl como paquete seguro cifrado (.bvda.pack).
    pub async fn cmd_export_secure_package(
        &self,
        dest_path: &str,
        password: String,
    ) -> Result<(), String> {
        let engine = self.get_engine()?;
        let secret_pass = SecretString::new(password);
        let package_json = engine
            .export_vault(&secret_pass)
            .await
            .map_err(|e| format!("Export failed: {}", e))?;

        std::fs::write(dest_path, package_json)
            .map_err(|e| format!("Failed to write export file: {}", e))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(dest_path, std::fs::Permissions::from_mode(0o600));
        }

        Ok(())
    }

    /// Importa un paquete seguro (.bvda.pack) al baúl actual.
    pub async fn cmd_import_secure_package(
        &self,
        src_path: &str,
        password: String,
        strategy: ImportStrategy,
    ) -> Result<(), String> {
        let engine = self.get_engine()?;
        let package_json = std::fs::read_to_string(src_path)
            .map_err(|e| format!("Failed to read import file: {}", e))?;
        let secret_pass = SecretString::new(password);
        engine
            .import_vault(&package_json, &secret_pass, strategy)
            .await
            .map_err(|e| format!("Import failed: {}", e))
    }

    /// Prepara la importación de una base de datos: cierra el pool, copia archivos.
    /// El llamador (Tauri) debe ejecutar `app.restart()` después de esta llamada.
    pub async fn cmd_prepare_import_db(&self, src_path: &str) -> Result<(), String> {
        let src = std::path::Path::new(src_path);
        if !src.exists() {
            return Err("El archivo de origen no existe".to_string());
        }

        let db_path = Self::vault_db_path();
        if src == db_path {
            return Err(
                "No puedes importar el mismo archivo que la app está usando. Usa 'Exportar' para respaldos."
                    .to_string(),
            );
        }

        // Cierra el pool para liberar el lock antes de sobrescribir
        let engine = {
            let mut engine_lock = self.engine.lock().unwrap();
            engine_lock.take()
        };
        if let Some(e) = engine {
            e.close().await;
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        // Elimina archivos WAL/SHM para evitar corrupción
        let _ = std::fs::remove_file(db_path.with_extension("bvda-wal"));
        let _ = std::fs::remove_file(db_path.with_extension("bvda-shm"));

        let src_salt = src.with_extension(format!(
            "{}.salt",
            src.extension().and_then(|e| e.to_str()).unwrap_or("bvda")
        ));
        let dest_salt = db_path.with_file_name("vault.salt");

        if src_salt.exists() {
            std::fs::copy(&src_salt, &dest_salt)
                .map_err(|e| format!("Error al copiar el salt: {}", e))?;
        } else {
            let _ = std::fs::remove_file(&dest_salt);
        }

        std::fs::copy(src_path, &db_path)
            .map_err(|e| format!("Error al copiar el archivo: {}", e))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&db_path, std::fs::Permissions::from_mode(0o600));
            if dest_salt.exists() {
                let _ = std::fs::set_permissions(&dest_salt, std::fs::Permissions::from_mode(0o600));
            }
        }

        Ok(())
    }

    // =========================================================================
    // 🔐 TOTP / SECURITY
    // =========================================================================

    pub async fn cmd_totp_is_enabled(&self) -> Result<bool, String> {
        let engine = self.get_engine()?;
        engine.is_totp_enabled().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_totp_setup(&self) -> Result<TotpSetupPayload, String> {
        let engine = self.get_engine()?;
        engine.setup_totp().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_totp_verify_setup(&self, code: &str) -> Result<bool, String> {
        let engine = self.get_engine()?;
        engine.verify_totp(code).await.map_err(|e| e.to_string())
    }

    pub async fn cmd_totp_check(&self, code: &str) -> Result<bool, String> {
        let engine = self.get_engine_unverified()?;
        let valid = engine.verify_totp(code).await.map_err(|e| e.to_string())?;
        if !valid {
            return Err("Código TOTP inválido".to_string());
        }
        *self.session_verified.lock().unwrap() = true;
        Ok(true)
    }

    pub async fn cmd_totp_recovery_check(&self, code: &str) -> Result<bool, String> {
        let engine = self.get_engine_unverified()?;
        let valid = engine
            .verify_totp_recovery(code)
            .await
            .map_err(|e| e.to_string())?;
        if !valid {
            return Err("Código de recuperación inválido o ya utilizado".to_string());
        }
        *self.session_verified.lock().unwrap() = true;
        Ok(true)
    }

    pub async fn cmd_totp_disable(&self) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine.disable_totp().await.map_err(|e| e.to_string())
    }

    // =========================================================================
    // 📋 AUDIT LOGS
    // =========================================================================

    pub async fn cmd_get_audit_logs(&self, limit: i64) -> Result<Vec<AuditLogView>, String> {
        let engine = self.get_engine()?;
        let logs = storage::get_audit_logs(&engine.db, limit)
            .await
            .map_err(|e| e.to_string())?;

        let view = logs
            .into_iter()
            .map(|(id, action, meta, created_at)| AuditLogView {
                id,
                action,
                metadata: meta,
                created_at,
            })
            .collect();

        Ok(view)
    }

    // =========================================================================
    // 📄 DOCUMENT MANAGEMENT
    // =========================================================================

    pub async fn cmd_add_document(
        &self,
        title: SecretString,
        description: Option<SecretString>,
        content: SecretString,
    ) -> Result<String, String> {
        let engine = self.get_engine()?;
        engine
            .add_document(title, description, content)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_get_documents(
        &self,
    ) -> Result<Vec<crate::storage::models::Document>, String> {
        let engine = self.get_engine()?;
        engine.get_documents().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_update_document(
        &self,
        id: &str,
        title: SecretString,
        description: Option<SecretString>,
        content: SecretString,
    ) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine
            .update_document(id, title, description, content)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_document(&self, id: &str) -> Result<(), String> {
        // SEC-C2: Validate UUID to prevent invalid IDs from being used
        Uuid::parse_str(id)
            .map_err(|_| format!("ID de documento inválido: '{}'. Debe ser un UUID válido.", id))?;

        let engine = self.get_engine()?;
        engine.delete_document(id).await.map_err(|e| e.to_string())
    }

    pub async fn cmd_decrypt_document_content(
        &self,
        encrypted_content: &str,
    ) -> Result<String, String> {
        let engine = self.get_engine()?;
        // SEC: Log access to decrypted document content
        let _ = engine.log_audit(crate::audit::AuditAction::SecretAccess, Some("document_content")).await;
        engine
            .decrypt_document_content(encrypted_content)
            .map_err(|e| e.to_string())
    }

    // =========================================================================
    // 🔧 INTERNAL HELPERS
    // =========================================================================

    fn app_data_dir() -> PathBuf {
        dirs_next::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("boveda")
    }

    pub fn vault_db_path() -> PathBuf {
        Self::app_data_dir().join("vault.bvda")
    }
}

// ─── View Types ───────────────────────────────────────────────────────────────

pub struct AuditLogView {
    pub id: i64,
    pub action: String,
    pub metadata: Option<String>,
    pub created_at: String,
}
