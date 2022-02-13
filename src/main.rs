use {
    crate::{
        config::*,
        pack::*,
    }
};

fn main() {
    let mut local = LocalModPack::new(
        "Magical".to_owned(),
        "1.12.2".to_owned(),
        "Forge".to_owned()
    );

    local.mods.push(
        LocalMod::CurseForge(CurseForgeSource{
            id: 1,
            date: 1
        })
    );

    local.store();
}

mod config;
mod pack;
