use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: Option<std::time::Duration>,
}

impl Track {
    pub fn from_path(path: PathBuf) -> Self {
        let title = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        Self {
            path,
            title: title.clone(),
            artist: "Unknown Artist".to_string(),
            album: "Unknown Album".to_string(),
            duration: None,
        }
    }

    pub fn display_name(&self) -> String {
        if self.artist != "Unknown Artist" {
            format!("{} - {}", self.artist, self.title)
        } else {
            self.title.clone()
        }
    }
}
