use {
    serde::Deserialize
};

pub trait ModExt {
    fn search_mods<'vm>(&'vm self, version: &str, 
                       loader: Option<&str>) -> Vec<&'vm ModFile>;
    fn where_mods(self, version: &str,
                  loader: Option<&String>) -> Vec<ModFile>;
    
    fn latest<'vm>(&'vm self) -> &'vm ModFile;
}

impl ModExt for Vec<ModFile> {
    fn search_mods<'vm>(&'vm self, version: &str, 
                       loader: Option<&str>) -> Vec<&'vm ModFile> {
        self.iter().filter(
            move |mf| mf.has_version(version) &&
                      if loader.is_some() { mf.has_mod_loader(loader.unwrap()) } else { true }
        ).collect()
    }

    fn where_mods(self, version: &str,
                  loader: Option<&String>) -> Vec<ModFile> {
        let mut results = vec![];
        for v in self {
            if loader.is_some() && !v.has_mod_loader(loader.unwrap()) {
                continue;
            }
            if !v.has_version(version) {
                continue;
            }

            results.push(v);
        }

        results
    }

    fn latest<'vm>(&'vm self) -> &'vm ModFile {
        self.iter().max_by(
            |x, y| x.id.cmp(&y.id)
        ).as_ref().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct GameVersion {
    pub version: String,
    pub mod_loader: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct ModFile {
    pub id: usize,

    #[serde(rename = "fileName")]
    pub filename: String,

    #[serde(rename = "gameVersion")]
    pub versions: Vec<String>,

    #[serde(rename = "downloadUrl")]
    pub download_url: String
}

#[derive(Deserialize, Debug)]
pub struct Mod {
    pub id: usize,

    pub name: String,
    pub summary: String,

    #[serde(rename = "websiteUrl")]
    pub curseforge: String,

    #[serde(rename = "modLoaders")]
    pub mod_loaders: Option<Vec<String>>,

    #[serde(rename = "latestFiles")]
    pub latest_files: Vec<ModFile>
}

impl GameVersion {
    pub fn new(version: String) -> Self {
        GameVersion{version, mod_loader: None}
    }

    pub fn with_loader(mut self, loader: String) -> Self {
        self.mod_loader = Option::Some(loader);
        self
    }
}

impl ModFile {
    pub fn has_version(&self, ver: &str) -> bool {
        let lower = ver.to_lowercase();
        self.versions.iter().any(move |v| v.to_lowercase() == lower)
    }

    pub fn has_mod_loader(&self, loader: &str) -> bool {
        let lower = loader.to_lowercase();
        if self.versions.len() == 1 {
            return true;
        } else if self.versions.is_empty() {
            return false;
        }

        self.versions.first().unwrap().to_lowercase() == lower
    }
}
