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
    #[serde(default)]
    pub mod_loaders: Vec<String>,

    #[serde(rename = "websiteUrl")]
    pub curseforge_url: String,
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
        let mut mod_loader_found = false;
        for version in self.versions.iter() {
            if is_mod_loader(version) {
                mod_loader_found = true;

                if loader == version {
                    return true;
                }
            }
        }

        !mod_loader_found
    }

    pub fn has_version(&self, version: &str) -> bool {
        self.versions.iter().any(move |v| v == version)
    }
}
