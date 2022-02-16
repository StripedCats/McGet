/*
 * Database for modpacks
 */

use {
    serde::{Deserialize, Serialize},
    crate::{
        config::*,
        fmt::*,
    },
    colored::*,
    chrono::{DateTime, Utc},

    std::path::{PathBuf, Path},
    symlink::*,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl UrlSource {
    pub fn basename(&self) -> String {
        Path::new(&self.url).file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_owned()
    }
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

    pub fn to_url_source(&self) -> UrlSource {
        match self {
            Self::Url(source) => source.clone(),
            _ => {
                panic!("Not an URL source ({:?})", self);
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

    pub fn load_from_database(
        name: &str
    ) -> Self {
        let buf = get_config_location().join("database").join(format!("{}.yaml", name));
        Self::load_file(
            buf.to_str().unwrap()
        )
    }

    #[inline]
    pub fn get_modpack_path(name: &str, create: &mut bool) -> PathBuf {
        let path = get_config_location().join("mods").join(name);
        if !path.exists() {
            if *create {
                if let Err(e) = std::fs::create_dir(&path) {
                    eprintln!("Can't make directory {:?}: {}", path, e);
                    panic_log_above();
                }

                *create = false;

                return path;
            }
            eprintln!("{}: modpack {} does not exists", "ERROR".red(), name.red());
            panic_log_above();
        }

        path
    }

    #[inline]
    pub fn switch_to_modpack(name: &str) {
        let path = Self::get_modpack_path(name, &mut false);
        let mods = Config::load().minecraft_path.join("mods");

        if mods.exists() && !mods.is_symlink() {
            println!("Can't switch modpack because mods is a non-symbolic link. Select an option");
            let result = ask_for(&[
                "Move mods directory to mods.mcget_backup",
                "Remove mods directory",
                "Exit"
            ]);

            match result {
                0 => {
                    if let Err(e) = std::fs::rename(&mods, mods.parent().unwrap().join("mods.mcget_backup")) {
                        eprintln!("{} to move directory {:?}: {}", "Failed".red(), mods, e);
                        panic_log_above();
                    }
                },

                1 => {
                    if let Err(e) = std::fs::remove_dir_all(&mods) {
                        eprintln!("{} to remove directory {:?}: {}", "Failed".red(), mods, e);
                        panic_log_above();
                    }
                },

                2 => panic_log_above(),

                _ => { unreachable!(); }
            }
        }

        remove_symlink_dir(&mods).unwrap_or_default();
        if let Err(e) = symlink_dir(&path, &mods) {
            eprintln!("{}: Can't make symbolic link {:?} => {:?} ({})", "ERROR".red(), &path, &mods, e);
            panic_log_above();
        }

        println!("{} made symbolic link {:?} => {:?}", "Successfully".bright_green(), path, mods);
    }

    pub fn remove_modpack(name: &str) {
        let mods = Config::load().minecraft_path.join("mods");
        let path = Self::get_modpack_path(name, &mut false);
        
        if let Err(e) = std::fs::remove_dir_all(&path) {
            eprintln!("{}: Can't remove directory {:?} ({})", "ERROR".red(), &path, e);
            panic_log_above();
        }

        remove_symlink_dir(mods).unwrap_or_default();
        println!("{} modpack {}", "Removed".red(), name.bright_red());
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
