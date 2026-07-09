// Export & Import-------------------------------------------------------------------------

use crate::crypto::secret::SecretString;
use crate::error::{BovedaError, BovedaResult};
use crate::storage;
use super::{BovedaEngine, ImportStrategy, export};

impl BovedaEngine {
    /// Exports the entire vault into a secure, encrypted package.
    pub async fn export_vault(&self, export_password: &SecretString) -> BovedaResult<String> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::VaultExport, None).await?;
        
        // 1. Get all accounts (DECRYPTED)
        let accounts = self.get_accounts().await?;
        let mut export_accounts = Vec::with_capacity(accounts.len());
        
        for acc in accounts {
            // Decrypt password, recovery code and notes
            let password = self.decrypt_secret(&acc.password_cipher)?;
            let recovery_code = acc.recovery_code_cipher.as_ref()
                .map(|c| self.decrypt_secret(c))
                .transpose()?;
            let notes = acc.notes_cipher.as_ref()
                .map(|c| self.decrypt_secret(c))
                .transpose()?;
                
            export_accounts.push(export::ExportAccount {
                site: acc.site.clone(),
                username: acc.username.clone(),
                password: password.as_str().to_string(),
                recovery_code: recovery_code.map(|s| s.as_str().to_string()),
                notes: notes.map(|s| s.as_str().to_string()),
                group_name: acc.group_name,
            });
        }
        
        // 1.5 Get all pins (DECRYPTED)
        let pins = self.get_pins().await?;
        let mut export_pins = Vec::with_capacity(pins.len());
        
        for p in pins {
            let notes = p.encrypted_notes.as_ref()
                .map(|c| self.decrypt_secret(c))
                .transpose()?;
                
            export_pins.push(export::ExportPin {
                name: p.name.clone(),
                pin: self.decrypt_secret(&p.encrypted_pin)?.as_str().to_string(),
                notes: notes.map(|s| s.as_str().to_string()),
            });
        }
        
        // 1.75 Get all documents (DECRYPTED)
        let documents = self.get_documents().await?;
        let mut export_documents = Vec::with_capacity(documents.len());
        
        for d in documents {
            let description = d.encrypted_description.as_ref()
                .map(|c| self.decrypt_document_content(c))
                .transpose()?;
                
            let content = self.decrypt_document_content(&d.encrypted_content)?;
                
            export_documents.push(export::ExportDocument {
                title: d.title.clone(),
                description,
                content,
            });
        }
        
        // 2. Get all preferences
        let all_preferences = storage::get_all_preferences(&self.db).await?;
        
        // Filter out TOTP configuration so it's not exported
        let preferences: Vec<(String, String)> = all_preferences
            .into_iter()
            .filter(|(k, _)| !k.starts_with("totp_"))
            .collect();
        
        let payload = export::ExportPayload {
            accounts: export_accounts,
            pins: export_pins,
            documents: export_documents,
            preferences,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        // 3. Encrypt into package
        let package = export::ExportPackage::encrypt(&payload, export_password)?;
        
        // 4. Serialize package to JSON
        serde_json::to_string(&package)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))
    }

    /// Imports a secure package into the current vault using the specified strategy.
    pub async fn import_vault(&self, package_json: &str, export_password: &SecretString, strategy: ImportStrategy) -> BovedaResult<()> {
        self.check_unlocked()?;
        self.log_audit(crate::audit::AuditAction::VaultImport, Some(match strategy {
            ImportStrategy::Merge => "merge",
            ImportStrategy::Replace => "replace",
        })).await?;

        // 1. Parse and decrypt package
        let package: export::ExportPackage = serde_json::from_str(package_json)
            .map_err(|e| BovedaError::SerializationError(e.to_string()))?;
        
        // SEC-6: Strictly validate version
        if package.version != 1 {
            return Err(BovedaError::Other(format!("Versión de paquete no soportada: {}", package.version)));
        }
        
        let payload = package.decrypt(export_password)?;

        // 2. Apply strategy
        if matches!(strategy, ImportStrategy::Replace) {
            // Clear current accounts
            sqlx::query("DELETE FROM accounts").execute(&self.db).await?;
            // Clear current pins
            sqlx::query("DELETE FROM pins").execute(&self.db).await?;
            // Clear current documents
            sqlx::query("DELETE FROM documents").execute(&self.db).await?;
            // Note: Preferences are overwritten anyway by set_preference later, 
            // but we might want to clear them too if we want a full replacement.
            // For now, let's just clear accounts as that's what "duplicates" refers to.
        }

        // 3. Insert accounts
        // We use add_account which handles encryption with the CURRENT master key.
        let existing_accounts = if matches!(strategy, ImportStrategy::Merge) {
            self.get_accounts().await?
        } else {
            vec![]
        };

        for acc in payload.accounts {
            // E-5: Deduplication check in Merge mode
            if matches!(strategy, ImportStrategy::Merge) {
                let duplicate = existing_accounts.iter().any(|existing| {
                    existing.site.as_str() == acc.site.as_str() && existing.username.as_str() == acc.username.as_str()
                });
                if duplicate {
                    continue; // Skip existing entry
                }
            }

            let id = self.add_account(
                acc.site.into(),
                acc.username.into(),
                acc.password.into(),
                acc.recovery_code.map(Into::into),
                acc.notes.map(Into::into)
            ).await?;
            if let Some(group) = acc.group_name {
                let _ = self.update_account_group(&id, Some(&group)).await;
            }
        }

        // 3.5 Insert pins
        let existing_pins = if matches!(strategy, ImportStrategy::Merge) {
            self.get_pins().await?
        } else {
            vec![]
        };

        for p in payload.pins {
            if matches!(strategy, ImportStrategy::Merge) {
                let duplicate = existing_pins.iter().any(|existing| {
                    existing.name.as_str() == p.name.as_str()
                });
                if duplicate {
                    continue; // Skip existing entry
                }
            }
            self.add_pin(p.name.into(), p.pin.into(), p.notes.map(Into::into)).await?;
        }

        // 3.75 Insert documents
        let existing_documents = if matches!(strategy, ImportStrategy::Merge) {
            self.get_documents().await?
        } else {
            vec![]
        };

        for d in payload.documents {
            if matches!(strategy, ImportStrategy::Merge) {
                let duplicate = existing_documents.iter().any(|existing| {
                    existing.title.as_str() == d.title.as_str()
                });
                if duplicate {
                    continue; // Skip existing entry
                }
            }
            self.add_document(
                d.title.into(),
                d.description.map(Into::into),
                d.content.into(),
            ).await?;
        }

        // 4. Apply preferences (Optional merge)
        // Skip TOTP preferences to avoid overwriting security state
        for (key, value) in payload.preferences {
            if !key.starts_with("totp_") {
                let _ = self.set_preference(&key, &value).await;
            }
        }

        Ok(())
    }

    // ─── Connection Management ─────────────────────────────────────────────────

    pub async fn close(&self) {
        self.db.close().await;
    }
}