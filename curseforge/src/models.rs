use {
    serde::Deserialize,
    chrono::{DateTime, Utc}
};

#[derive(Debug, Deserialize)]
pub struct ModEntry {
    pub id: i64,
    pub name: String,
    pub summary: String,

    #[serde(rename = "modLoaders")]
    pub mod_loaders: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModFile {
    pub id: i64,

    #[serde(rename = "downloadUrl")]
    pub download_url: String,

    #[serde(rename = "fileDate")]
    pub date: DateTime<Utc>,

    #[serde(rename = "gameVersion")]
    pub versions: Vec<String>,
}

#[inline]
pub fn is_mod_loader(loader: &str) -> bool {
    match loader.to_lowercase().as_str() {
        "forge" | "fabric" | "liteloader" => true,
        _ => false,
    }
}

impl ModFile {
    pub fn has_loader(&self, loader: &str) -> bool {
        self.versions.iter()
                     .filter(|v| is_mod_loader(&v))
                     .any(|v| v == loader)
    }

    pub fn has_version(&self, version: &str) -> bool {
        self.versions.iter().any(move |v| v == version)
    }
}
