use crate::crypto::secret::SecretString;
use uuid::Uuid;

impl super::AppState {
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

    pub async fn cmd_get_documents(&self) -> Result<Vec<crate::storage::models::Document>, String> {
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
        Uuid::parse_str(id)
            .map_err(|_| format!("Invalid document ID: '{}'. Must be a valid UUID.", id))?;

        let engine = self.get_engine()?;
        engine.delete_document(id).await.map_err(|e| e.to_string())
    }

    pub async fn cmd_decrypt_document_content(&self, encrypted_content: &str) -> Result<String, String> {
        let engine = self.get_engine()?;
        // SEC: Log access to decrypted document content
        let _ = engine.log_audit(crate::audit::AuditAction::SecretAccess, Some("document_content")).await;
        engine
            .decrypt_document_content(encrypted_content)
            .map_err(|e| e.to_string())
    }
}
