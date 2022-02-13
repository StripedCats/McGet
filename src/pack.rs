/*
 * Database for modpacks
 */

use {
    serde::{Deserialize, Serialize},
    crate::config::*,
    colored::*,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct UrlSource {
    #[serde(rename = "Url")]
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurseForgeSource {
    #[serde(rename = "Id")]
    pub id: i64,

    #[serde(rename = "PublishDate")]
    pub date: i32,
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

impl LocalModPack {
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

    pub fn load(
        name: &str
    ) -> Self {
        let path = get_config_location().join("database").join(format!("{}.yaml", name));
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

    pub fn store(&self) {
        let text = serde_yaml::to_string(self).unwrap();
        let path = get_config_location().join("database").join(format!("{}.yaml", self.name));

        if let Err(e) = std::fs::write(path, text) {
            eprintln!("{} to write local modpack information: {}", "Failed".red(), e);
            panic_log_above();
        }
    }
}
