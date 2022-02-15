/*
 * Database for modpacks
 */

use {
    serde::{Deserialize, Serialize},
    crate::config::*,
    colored::*,
    chrono::{DateTime, Utc},

    std::path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UrlSource {
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurseForgeSource {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "PublishDate")]
    pub date: Option<DateTime<Utc>>,

    #[serde(rename = "FileName")]
    pub filename: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LocalMod {
    Url(UrlSource),
    CurseForge(CurseForgeSource)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalModPack {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Mods")]
    pub mods: Vec<LocalMod>,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "ModLoader")]
    pub loader: String,
}

impl LocalMod {
    pub fn is_curseforge(&self) -> bool {
        match self {
            Self::CurseForge(_) => true,
            _ => false,
        }
    }

    pub fn to_curseforge_source(&self) -> CurseForgeSource {
        match self {
            Self::CurseForge(source) => source.clone(),
            _ => {
                panic!("Not a curseforge source ({:?})", self);
            }
        }
    }
}

impl LocalModPack {
    #[inline]
    pub fn by_id(
        &self,
        id: i64,
    ) -> Option<&LocalMod> {
        self.mods.iter()
                 .find(move |m| match m {
                     LocalMod::CurseForge(source) => source.id == id,
                     _ => false,
                 })
    }

    #[inline]
    pub fn by_id_mut(
        &mut self,
        id: i64
    ) -> Option<&mut LocalMod> {
        self.mods.iter_mut()
                 .find(move |m| match m {
                     LocalMod::CurseForge(source) => source.id == id,
                     _ => false,
                 })
    }

    pub fn new(
        name: String,
        version: String,
        loader: String,
    ) -> Self {
        Self{
            mods: Default::default(),

            name,
            version,
            loader
        }
    }

    #[inline]
    pub fn load_from_database(
        name: &str
    ) -> Self {
        let buf = get_config_location().join("database").join(format!("{}.yaml", name));
        Self::load_file(
            buf.to_str().unwrap()
        )
    }

    pub fn load_file(
        name: &str
    ) -> Self {
        let path = PathBuf::from(name);
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{} to read modpack {}: {}", "Failed".red(), name.cyan(), e);
                panic_log_above();
            }
        };

        match serde_yaml::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{} to read modpack {}: {}", "Failed".red(), name.cyan(), e);
                panic_log_above();
            }
        }
    }

    #[inline]
    pub fn store_to_database(&self) {
        self.store_to_file(
            get_config_location().join("database")
                                      .join(format!("{}.yaml", self.name))
                                      .to_str()
                                      .unwrap()
        );
    }

    pub fn store_to_file(&self, path: &str) {
        let text = serde_yaml::to_string(self).unwrap();

        if let Err(e) = std::fs::write(path, text) {
            eprintln!("{} to write local modpack information: {}", "Failed".red(), e);
            panic_log_above();
        }
    }
}
