use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct ModFile {
    #[serde(alias = "fileName")]
    pub file_name: String,

    #[serde(alias = "gameVersion")]
    pub versions: Vec<String>,

    #[serde(alias = "downloadUrl")]
    pub download_url: String
}

#[derive(Deserialize, Debug)]
pub struct Mod {
    pub id: usize,
    
    pub name: String,
    pub summary: String,

    #[serde(alias = "websiteUrl")]
    pub curseforge: String,

    #[serde(alias = "modLoaders")]
    pub mod_loaders: Option<Vec<String>>
}

impl ModFile {
    pub fn contains_mod_loader(&self, loader: &str) -> bool {
        let lower = loader.to_lowercase();
        
        for version in &self.versions {
            let ver = version.to_lowercase();

            if ver == lower {
                return true;
            } else if &ver == "forge" || &ver == "fabric" {
                return false;
            }
        }

        true
    }

    pub fn contains_version(&self, ver: &str) -> bool {
        for version in &self.versions {
            if version.to_lowercase() == ver.to_lowercase() {
                return true;
            }
        }
        false
    }
}
