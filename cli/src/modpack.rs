use {
    serde::{Serialize, Deserialize}
};

#[derive(Serialize, Deserialize)]
pub struct ModpackMod {
    #[serde(rename = "Id")]
    pub id: usize
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
        self.mods.iter().any(move |v| v.id == id)
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
    pub fn new(id: usize) -> ModpackMod {
        ModpackMod{id}
    }
}
