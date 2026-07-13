use crate::crypto;
use crate::crypto::secret::SecretString;
use crate::error::{BovedaError, BovedaResult};
use crate::storage;
use crate::vault::validation;

impl super::BovedaEngine {
    pub async fn get_pins(&self) -> BovedaResult<Vec<crate::storage::models::Pin>> {
        self.check_unlocked()?;
        let rows = storage::get_pins(&self.db).await?;
        let mut pins = Vec::with_capacity(rows.len());
        for row in rows {
            let dec_name = self.with_key(|key| {
                // SEC-D1: Propagate decryption errors — do not return the ciphertext blob.
                crypto::decrypt(&row.name, key)
            })??;

            pins.push(crate::storage::models::Pin {
                id: row.id,
                name: dec_name.as_str().to_string(),
                encrypted_pin: row.encrypted_pin,
                encrypted_notes: row.encrypted_notes,
                group_name: row.group_name,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        pins.sort_by_key(|p| p.name.to_lowercase());
        Ok(pins)
    }

    pub async fn add_pin(
        &self,
        name: SecretString,
        pin: SecretString,
        notes: Option<SecretString>,
    ) -> BovedaResult<String> {
        self.check_unlocked()?;

        validation::validate_string(name.as_str(), "Nombre", validation::MAX_PIN_NAME_LEN, true)?;
        validation::validate_string(pin.as_str(), "PIN", validation::MAX_PIN_LEN, true)?;
        if let Some(n) = &notes {
            validation::validate_string(n.as_str(), "Notas", validation::MAX_NOTES_LEN, false)?;
        }

        let (enc_name, enc_pin, enc_notes) = self.with_key(|key| {
            let n = crypto::encrypt(&name, key)?;
            let p = crypto::encrypt(&pin, key)?;
            let note = notes.as_ref().map(|n| crypto::encrypt(n, key)).transpose()?;
            Ok::<_, BovedaError>((n, p, note))
        })??;

        let id = storage::add_pin(
            &self.db,
            &enc_name,
            &enc_pin,
            enc_notes.as_deref(),
            None,
        ).await?;

        self.log_audit(crate::audit::AuditAction::PinCreate, Some(&id)).await?;
        Ok(id)
    }

    pub async fn delete_pin(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::PinDelete, Some(id)).await?;
        storage::delete_pin(&self.db, id).await
    }

    pub async fn update_pin_group(&self, id: &str, group_name: Option<&str>) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::PinGroupUpdate, Some(id)).await?;
        storage::update_pin_group(&self.db, id, group_name).await
    }
}
