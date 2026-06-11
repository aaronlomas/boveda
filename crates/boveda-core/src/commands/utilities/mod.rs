// Only utilities for background images and paths remain here.

impl super::AppState {
    pub async fn cmd_import_background_image(src_path: &str) -> Result<String, String> {
        let src = std::path::Path::new(src_path);
        if !src.exists() {
            return Err("The image file does not exist.".to_string());
        }

        let ext = src
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("jpg")
            .to_lowercase();

        let allowed = ["jpg", "jpeg", "png", "webp", "gif"];
        if !allowed.contains(&ext.as_str()) {
            return Err("Image format not supported. Use JPG, PNG, WEBP or GIF.".to_string());
        }

        let dest_dir = Self::app_data_dir();
        for old_ext in &allowed {
            let _ = std::fs::remove_file(dest_dir.join(format!("background.{}", old_ext)));
        }

        let dest_filename = format!("background.{}", ext);
        let dest = dest_dir.join(&dest_filename);
        std::fs::copy(src_path, &dest)
            .map_err(|e| format!("Error to copy image: {}", e))?;

        Ok(dest_filename)
    }

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
            "png" => "image/png",
            "gif" => "image/gif",
            "webp" => "image/webp",
            _ => "image/jpeg",
        };

        use base64::Engine as _;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);
        Ok(format!("data:{};base64,{}", mime, encoded))
    }

    pub fn cmd_get_data_dir() -> String {
        Self::app_data_dir().to_string_lossy().to_string()
    }
}
