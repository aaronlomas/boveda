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

// ─── AppState ─────────────────────────────────────────────────────────────────

/// Estado global compartido entre todos los manejadores de comandos.
#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<Mutex<Option<BovedaEngine>>>,
    pub db_path: PathBuf,
}

impl AppState {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            engine: Arc::new(Mutex::new(None)),
            db_path,
        }
    }

    /// Obtiene el engine o devuelve error si el baúl está bloqueado.
    fn get_engine(&self) -> Result<BovedaEngine, String> {
        let lock = self.engine.lock().unwrap();
        lock.as_ref().cloned().ok_or_else(|| "El baúl está bloqueado".to_string())
    }

    // =========================================================================
    // 🔒 VAULT LIFECYCLE
    // =========================================================================

    pub fn cmd_is_vault_initialized(&self) -> bool {
        BovedaEngine::is_initialized(&self.db_path)
    }

    /// Desbloquea el baúl. Devuelve `"totp_required"` o `"unlocked"`.
    pub async fn cmd_unlock_vault(&self, password: SecretString) -> Result<String, String> {
        let engine = BovedaEngine::unlock(&self.db_path, &password)
            .await
            .map_err(|e| e.to_string())?;

        let is_totp = engine.is_totp_enabled().await.unwrap_or(false);

        let mut engine_lock = self.engine.lock().unwrap();
        *engine_lock = Some(engine);

        if is_totp {
            Ok("totp_required".to_string())
        } else {
            Ok("unlocked".to_string())
        }
    }

    pub fn cmd_lock_vault(&self) {
        let mut lock = self.engine.lock().unwrap();
        if let Some(engine) = lock.as_ref() {
            engine.lock();
        }
        *lock = None;
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
        engine.get_pins().await.map_err(|e| e.to_string())
    }

    pub async fn cmd_delete_pin(&self, id: &str) -> Result<(), String> {
        let engine = self.get_engine()?;
        engine.delete_pin(id).await.map_err(|e| e.to_string())
    }

    /// Genera una contraseña aleatoria (nunca se almacena).
    pub fn cmd_generate_password(length: usize, use_symbols: bool) -> String {
        let len = length.clamp(8, 128);
        crypto::generate_password(len, use_symbols).as_str().to_string()
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
        let engine = self.get_engine()?;
        let valid = engine.verify_totp(code).await.map_err(|e| e.to_string())?;
        if !valid {
            return Err("Código TOTP inválido".to_string());
        }
        Ok(true)
    }

    pub async fn cmd_totp_recovery_check(&self, code: &str) -> Result<bool, String> {
        let engine = self.get_engine()?;
        let valid = engine
            .verify_totp_recovery(code)
            .await
            .map_err(|e| e.to_string())?;
        if !valid {
            return Err("Código de recuperación inválido o ya utilizado".to_string());
        }
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
