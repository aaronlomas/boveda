impl super::AppState {
	pub async fn cmd_totp_is_enabled(&self) -> Result<bool, String> {
		let engine = self.get_engine()?;
		engine.is_totp_enabled().await.map_err(|e| e.to_string())
	}

	pub async fn cmd_totp_setup(&self) -> Result<crate::auth::TotpSetupPayload, String> {
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
			return Err("Invalid TOTP code".to_string());
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
			return Err("Invalid or already used recovery code".to_string());
		}
		*self.session_verified.lock().unwrap() = true;
		Ok(true)
	}

	pub async fn cmd_totp_disable(&self) -> Result<(), String> {
		let engine = self.get_engine()?;
		engine.disable_totp().await.map_err(|e| e.to_string())
	}
}
