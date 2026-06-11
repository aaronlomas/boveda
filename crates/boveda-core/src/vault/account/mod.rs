use crate::crypto;
use crate::crypto::secret::SecretString;
use crate::error::{BovedaError, BovedaResult};
use crate::storage;
use crate::vault::validation;

impl super::BovedaEngine {
    /// Retrieves and decrypts all accounts in the vault.
    pub async fn get_accounts(&self) -> BovedaResult<Vec<crate::storage::models::Account>> {
        self.check_unlocked()?;
        let rows = storage::get_accounts(&self.db).await?;
        self.decrypt_rows(rows)
    }

    /// Retrieves and decrypts a page of accounts.
    pub async fn get_accounts_paged(&self, limit: i64, offset: i64) -> BovedaResult<Vec<crate::storage::models::Account>> {
        self.check_unlocked()?;
        let rows = storage::get_accounts_paged(&self.db, limit, offset).await?;
        self.decrypt_rows(rows)
    }

    /// Helper to decrypt a batch of account rows.
    fn decrypt_rows(&self, rows: Vec<crate::storage::AccountRow>) -> BovedaResult<Vec<crate::storage::models::Account>> {
        let mut accounts = Vec::with_capacity(rows.len());
        for row in rows {
            let (dec_site, dec_username) = self.with_key(|key| {
                let s = crypto::decrypt(&row.site, key)?;
                let u = crypto::decrypt(&row.username, key)?;
                Ok::<_, BovedaError>((s, u))
            })??;

            accounts.push(crate::storage::models::Account {
                id: row.id,
                site: dec_site.as_str().to_string(),
                username: dec_username.as_str().to_string(),
                password_cipher: row.encrypted_password,
                recovery_code_cipher: row.encrypted_recovery_code,
                notes_cipher: row.encrypted_notes,
                favicon_url: row.favicon_url,
                group_name: row.group_name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }

        accounts.sort_by_key(|a| a.site.to_lowercase());
        Ok(accounts)
    }

    /// Adds a new account, encrypting sensitive fields automatically.
    pub async fn add_account(
        &self,
        site: SecretString,
        username: SecretString,
        password: SecretString,
        recovery_code: Option<SecretString>,
        notes: Option<SecretString>,
    ) -> BovedaResult<String> {
        self.check_unlocked()?;

        validation::validate_string(site.as_str(), "Sitio", validation::MAX_SITE_LEN, true)?;
        validation::validate_string(username.as_str(), "Usuario", validation::MAX_USERNAME_LEN, true)?;
        validation::validate_string(password.as_str(), "Contraseña", validation::MAX_PASSWORD_LEN, true)?;
        if let Some(rc) = &recovery_code {
            validation::validate_string(rc.as_str(), "Código de recuperación", validation::MAX_PASSWORD_LEN, false)?;
        }
        if let Some(n) = &notes {
            validation::validate_string(n.as_str(), "Notas", validation::MAX_NOTES_LEN, false)?;
        }

        let (enc_site, enc_username, enc_password, enc_recovery_code, enc_notes) = self.with_key(|key| {
            let s = crypto::encrypt(&site, key)?;
            let u = crypto::encrypt(&username, key)?;
            let p = crypto::encrypt(&password, key)?;
            let rc = recovery_code.as_ref().map(|rc| crypto::encrypt(rc, key)).transpose()?;
            let n = notes.as_ref().map(|n| crypto::encrypt(n, key)).transpose()?;
            Ok::<_, BovedaError>((s, u, p, rc, n))
        })??;

        let id = storage::add_account(
            &self.db,
            &enc_site,
            &enc_username,
            &enc_password,
            enc_recovery_code.as_deref(),
            enc_notes.as_deref(),
            None,
        ).await?;

        self.log_audit(crate::audit::AuditAction::AccountCreate, Some(&id)).await?;
        Ok(id)
    }

    /// Decrypts a single ciphertext on-demand.
    pub fn decrypt_secret(&self, ciphertext: &str) -> BovedaResult<SecretString> {
        self.with_key(|key| crypto::decrypt(ciphertext, key))?
    }

    /// Deletes an account by ID.
    pub async fn delete_account(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::AccountDelete, Some(id)).await?;
        storage::delete_account(&self.db, id).await
    }
}
