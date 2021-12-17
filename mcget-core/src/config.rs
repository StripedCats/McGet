use {
    serde::{Serialize, Deserialize},
    std::{
        path::Path
    },

    crate::cli_helpers::log,
    colored::*
};


#[derive(Serialize, Deserialize, Debug)]
pub struct ModConfig {
    #[serde(rename = "Id")]
    pub id: usize,

    // #[serde(rename = "Name")]
    // pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MinecraftConfig {
    #[serde(rename = "McVersion")]
    pub version: String,

    #[serde(rename = "ModLoader")]
    pub mod_loader: String,

    #[serde(rename = "Mods")]
    pub mods: Vec<ModConfig>,

    #[serde(rename = "ModpackName")]
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModpackConfig {
    #[serde(rename = "ModPack")]
    pub modpack: MinecraftConfig
}

impl ModpackConfig {
    pub fn lookup() -> ModpackConfig {
        if !Path::new("Modpack.yaml").exists() {
            log(&format!("{} config file not found in this directory", "Modpack.yaml".bold()));
            std::process::exit(1);
        }

        let content = std::fs::read_to_string("Modpack.yaml").unwrap();
        match serde_yaml::from_str(&content) {
            Ok(result) => result,
            Err(e) => {
                log(&format!(
                    "Failed to parse Modpack.yaml: {}",
                    e.to_string().bold()
                ));
                std::process::exit(1);
            }
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.modpack.name = name;
        self
    }

    pub fn store(&self) {
        match serde_yaml::to_string(self) {
            Ok(result) => {
                std::fs::write("Modpack.yaml", &result).unwrap();
                log("Modpack.yaml is saved");
            },

            Err(_) => {
                log(&format!(
                    "{} to write Modpack.yaml",
                    "Failed".red()
                ));
                std::process::exit(1);
            }
        }
    }
}

impl Default for ModpackConfig {
    fn default() -> Self {
        Self{modpack: MinecraftConfig{
            name: "".to_string(),
            version: "1.12.2".to_string(),
            mod_loader: "Forge".to_string(),
            mods: vec![]
        }}
    }
}
