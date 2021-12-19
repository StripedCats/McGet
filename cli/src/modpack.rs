use {
    serde::{Serialize, Deserialize}
};

#[derive(Serialize, Deserialize)]
pub struct ModpackMod {
    #[serde(rename = "Id")]
    id: usize
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

#[derive(Serialize, Deserialize)]
pub struct ModpackCfg {
    #[serde(skip)]
    pub file: String,

    #[serde(rename = "Minecraft")]
    pub mc: MinecraftModpack
}

impl ModpackMod {
    pub fn new(id: usize) -> ModpackMod {
        ModpackMod{id}
    }
}
