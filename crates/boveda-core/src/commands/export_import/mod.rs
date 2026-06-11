use crate::crypto::secret::SecretString;
use crate::vault::ImportStrategy;

impl super::AppState {
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

	pub async fn cmd_prepare_import_db(&self, src_path: &str) -> Result<(), String> {
		let src = std::path::Path::new(src_path);
		if !src.exists() {
			return Err("The source file does not exist".to_string());
		}

		let db_path = Self::vault_db_path();
		if src == db_path {
			return Err(
				"Cannot import the same file the app is using. Use 'Export' for backups."
					.to_string(),
			);
		}

		//Close the pool to release the lock before overwriting
		let engine = {
			let mut engine_lock = self.engine.lock().unwrap();
			engine_lock.take()
		};
		if let Some(e) = engine {
			e.close().await;
		}
		tokio::time::sleep(std::time::Duration::from_millis(200)).await;

		// Delete WAL/SHM files to prevent corruption
		let _ = std::fs::remove_file(db_path.with_extension("bvda-wal"));
		let _ = std::fs::remove_file(db_path.with_extension("bvda-shm"));

		let src_salt = src.with_extension(format!(
			"{}.salt",
			src.extension().and_then(|e| e.to_str()).unwrap_or("bvda")
		));
		let dest_salt = db_path.with_file_name("vault.salt");

		if src_salt.exists() {
			std::fs::copy(&src_salt, &dest_salt)
				.map_err(|e| format!("Error to copy salt: {}", e))?;
		} else {
			let _ = std::fs::remove_file(&dest_salt);
		}

		std::fs::copy(src_path, &db_path)
			.map_err(|e| format!("Error to copy file: {}", e))?;

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
}
