use crate::crypto;
use crate::crypto::secret::SecretString;
use crate::error::{BovedaError, BovedaResult};
use crate::storage;
use crate::vault::validation;

impl super::BovedaEngine {
    /// Retrieves and decrypts all document headers (title + metadata, NOT content).
    pub async fn get_documents(&self) -> BovedaResult<Vec<crate::storage::models::Document>> {
        self.check_unlocked()?;
        let rows = storage::get_documents(&self.db).await?;
        let mut docs = Vec::with_capacity(rows.len());
        for row in rows {
            let dec_title = self.with_key(|key| {
                crypto::decrypt(&row.title, key)
            })??;
            docs.push(crate::storage::models::Document {
                id: row.id,
                title: dec_title.as_str().to_string(),
                encrypted_description: row.encrypted_description,
                encrypted_content: row.encrypted_content,
                created_at: row.created_at,
                updated_at: row.updated_at,
            });
        }
        Ok(docs)
    }

    /// Adds a new encrypted document/note.
    pub async fn add_document(
        &self,
        title: SecretString,
        description: Option<SecretString>,
        content: SecretString,
    ) -> BovedaResult<String> {
        self.check_unlocked()?;

        validation::validate_string(title.as_str(), "Título", 256, true)?;
        if let Some(d) = &description {
            validation::validate_string(d.as_str(), "Descripción", 1024, false)?;
        }
        validation::validate_string(content.as_str(), "Contenido", 1_048_576, false)?;

        let (enc_title, enc_description, enc_content) = self.with_key(|key| {
            let t = crypto::encrypt(&title, key)?;
            let d = description
                .as_ref()
                .map(|d| crypto::encrypt(d, key))
                .transpose()?;
            let c = crypto::encrypt(&content, key)?;
            Ok::<_, BovedaError>((t, d, c))
        })??;

        let id = storage::add_document(
            &self.db,
            &enc_title,
            enc_description.as_deref(),
            &enc_content,
        )
        .await?;

        self.log_audit(crate::audit::AuditAction::DocumentCreate, Some(&id)).await?;
        Ok(id)
    }

    /// Updates an existing encrypted document/note.
    pub async fn update_document(
        &self,
        id: &str,
        title: SecretString,
        description: Option<SecretString>,
        content: SecretString,
    ) -> BovedaResult<()> {
        self.check_unlocked()?;

        validation::validate_string(title.as_str(), "Título", 256, true)?;
        if let Some(d) = &description {
            validation::validate_string(d.as_str(), "Descripción", 1024, false)?;
        }
        validation::validate_string(content.as_str(), "Contenido", 1_048_576, false)?;

        let (enc_title, enc_description, enc_content) = self.with_key(|key| {
            let t = crypto::encrypt(&title, key)?;
            let d = description
                .as_ref()
                .map(|d| crypto::encrypt(d, key))
                .transpose()?;
            let c = crypto::encrypt(&content, key)?;
            Ok::<_, BovedaError>((t, d, c))
        })??;

        storage::update_document(
            &self.db,
            id,
            &enc_title,
            enc_description.as_deref(),
            &enc_content,
        )
        .await?;

        self.log_audit(crate::audit::AuditAction::DocumentUpdate, Some(id)).await?;
        Ok(())
    }

    /// Deletes a document by ID.
    pub async fn delete_document(&self, id: &str) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::DocumentDelete, Some(id)).await?;
        storage::delete_document(&self.db, id).await
    }

    /// Decrypts the content of a single document on demand.
    pub fn decrypt_document_content(&self, encrypted_content: &str) -> BovedaResult<String> {
        self.with_key(|key| {
            crypto::decrypt(encrypted_content, key)
                .map(|s| s.as_str().to_string())
        })?
    }
}
