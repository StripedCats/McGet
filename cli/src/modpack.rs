use {
    serde::{Serialize, Deserialize}
};

#[derive(Serialize, Deserialize)]
pub struct ModpackMod {
    #[serde(rename = "Id")]
    pub id: Option<usize>,

    #[serde(rename = "Url")]
    pub url: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct MinecraftModpack {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "ModLoader")]
    pub loader: String,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Mods")]
    pub mods: Vec<ModpackMod>
}

impl MinecraftModpack {
    pub fn has_modid(&self, id: usize) -> bool {
        self.mods.iter().any(move |v| v.id.unwrap_or(0) == id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ModpackCfg {
    #[serde(skip)]
    pub file: String,

    #[serde(rename = "Minecraft")]
    pub mc: MinecraftModpack
}

impl ModpackMod {
    pub fn with_id(id: usize) -> ModpackMod {
        ModpackMod{id: Some(id), url: None}
    }

    pub fn with_url(url: String) -> ModpackMod {
        ModpackMod{id: None, url: Some(url)}
    }
}
