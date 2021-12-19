use {
    whoami::username,
    std::{
        path::{PathBuf},
    },
    serde::{Serialize, Deserialize},
    symlink::{symlink_dir, remove_symlink_dir},
    colored::*,

    crate::modpack::*
};

#[derive(Serialize, Deserialize)]
pub struct McGetConfig {
    #[serde(rename = "MinecraftPath")]
    minecraft_path: PathBuf,

    #[serde(skip)]
    modpacks: Vec<PathBuf>
}

impl McGetConfig {
    pub fn new(mc_path: String) -> McGetConfig {
        McGetConfig{minecraft_path: mc_path.into(), modpacks: Default::default()}
    }

    pub fn store(&self) {
        let loc = get_config_location().join("McGet.yaml");
        let string = serde_yaml::to_string::<Self>(self).unwrap();

        std::fs::write(&loc, string).unwrap_or_default();
    }

    pub fn find_modpack<'a>(&'a self, name: &str) -> &'a PathBuf {
        for pack in &self.modpacks {
            if pack.file_name().unwrap() == name {
                return pack;
            }
        }

        println!("Error: {} {}", "No pack named ".red(), name.bold());
        std::process::exit(1);
    }

    pub fn remove_modpack(&self, name: &str) {
        let pack = self.find_modpack(name);
        match std::fs::remove_dir_all(&pack) {
            Ok(_) => {
                println!("Successfully removed {}", pack.as_path().to_str().unwrap().bold());
            },

            Err(e) => {
                println!("Failed to remove {}: {}", pack.as_path().to_str().unwrap().bold(),
                         e.to_string());
                std::process::exit(1);
            }
        }
    }

    pub fn switch_modpack(&self, name: &str) {
        let pack = self.find_modpack(name);

        let mods = self.minecraft_path.join("mods");
        match remove_symlink_dir(&mods) {
            Ok(_) => {},
            Err(e) => {
                println!("{} Can't remove symlink for directory {}: {}", "!!WARNING".red(),
                         mods.to_str().unwrap(), e.to_string().red());
            }
        }

        match symlink_dir(&pack, &mods) {
            Ok(_) => {
                println!("Created symlink {} => {}", pack.as_path().to_str().unwrap().bold(),
                         mods.as_path().to_str().unwrap().bold());
            },
            Err(_) => {
                println!("Can't symlink {} => {}", pack.as_path().to_str().unwrap().bold(),
                         mods.as_path().to_str().unwrap().bold());
                std::process::exit(1);
            }
        }
    }

    pub fn lookup() -> McGetConfig {
        let path = get_config_location().join("McGet.yaml");
        if !path.exists() {
            let cfg = Self::new(default_minecraft_path());
            cfg.store();
        }

        let content = std::fs::read_to_string(&path).unwrap();
        let mut cfg: McGetConfig = serde_yaml::from_str(&content).unwrap();
        
        for dir in std::fs::read_dir(path.parent().unwrap().join("modpacks")).unwrap() {
            let entry;
            match dir {
                Ok(e) => { entry = e; },
                Err(_) => { break; }
            }

            cfg.modpacks.push(entry.path());
        }

        cfg
    }
}

impl ModpackCfg {
    pub fn new(name: String, version: String,
               loader: String, file: String) -> ModpackCfg {
        ModpackCfg{
            file,
            mc: MinecraftModpack{
                name, version,
                loader, mods: Default::default()
            }
        }
    }

    pub fn load(filename: &str) -> ModpackCfg {
        let contents = match std::fs::read_to_string(filename) {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e.to_string().red());
                std::process::exit(1);
            }
        };

        match serde_yaml::from_str::<ModpackCfg>(&contents) {
            Ok(mut r) => {
                r.file = filename.to_string();
                r
            },
            Err(e) => {
                println!("Failed to parse YAML file: {}", e.to_string().red());
                std::process::exit(1);
            }
        }
    }

    pub fn store(&self) {
        let content = serde_yaml::to_string(self).unwrap();
        std::fs::write(&self.file, content).unwrap_or_default();
    }
}

fn default_minecraft_path() -> String {
    let username = username();
    if cfg!(target_os = "linux") || cfg!(target_os = "freebsd")
       || cfg!(target_os = "openbsd") {
        format!("/home/{}/.minecraft", username)
    } else if cfg!(target_os = "macos") {
        format!("/Users/{}/Library/Application Support/minecraft", username)
    } else {
        format!("C:\\Users\\{}\\AppData\\.minecraft", username)
    }
}

#[inline(always)]
pub fn get_config_location() -> PathBuf {
    let username = username();
    let mut path = PathBuf::new();

    if cfg!(target_os = "linux") || cfg!(target_os = "freebsd")
       || cfg!(target_os = "openbsd") {
        if username == "root" {
            path.push("/root/.local/");
        } else {
            path.push(&format!("/home/{}/.local/", username));
        }
    } else if cfg!(target_os = "macos") {
        path.push(&format!("/Users/{}/.local/", username));
    } else {
        path.push(format!("C:\\Users\\{}\\AppData\\", username));
    }
    
    path.push("mcget");
    
    std::fs::create_dir(&path).unwrap_or_default();
    std::fs::create_dir(&path.join("modpacks")).unwrap_or_default();

    path
}
