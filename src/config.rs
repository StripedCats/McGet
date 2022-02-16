use {
    whoami::username,
    std::{
        path::{PathBuf}, // , Path},
    },

    serde::{
        Serialize, Deserialize
    },

    colored::*,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "MinecraftPath")]
    pub minecraft_path: PathBuf,
}

impl Config {
    pub fn new(
        minecraft_path: String,
    ) -> Self {
        Self{
            minecraft_path: minecraft_path.into()
        }
    }

    pub fn load() -> Self {
        let path = get_config_location().join("mcget.yaml");
        if !path.exists() {
            Config::new(default_minecraft_path()).store();
        }

        let text = if let Ok(txt) = std::fs::read_to_string(&path) {
            txt
        } else {
            eprintln!("{} to read config file from {}", "Failed".red(), path.to_str()
                                                                            .unwrap()
                                                                            .red());
            panic_log_above();
        };


        if let Ok(read) = serde_yaml::from_str(&text) {
            read
        } else {
            eprintln!("{} to parse config file from {}", "Failed".red(), path.to_str()
                                                                             .unwrap()
                                                                             .red());
            panic_log_above();
        }
    }

    pub fn store(&self) {
        let text = serde_yaml::to_string(self).unwrap();
        let config_path = get_config_location().join("mcget.yaml");

        if std::fs::write(
            &config_path,
            &text
        ).is_err() {
            eprintln!("{} to write config file to path {}", "Failed".red(), config_path.to_str()
                                                                                       .unwrap()
                                                                                       .red());
            panic_log_above();
        }
    }
}

#[inline]
pub fn panic_log_above() -> ! {
    std::process::exit(1);
}

fn default_minecraft_path() -> String {
    let username = username();
    if cfg!(target_os = "linux") || cfg!(target_os = "freebsd")
       || cfg!(target_os = "openbsd") {
        format!("/home/{}/.minecraft", username)
    } else if cfg!(target_os = "macos") {
        format!("/Users/{}/Library/Application Support/minecraft", username)
    } else {
        format!("C:\\Users\\{}\\AppData\\Roaming\\.minecraft", username)
    }
}

#[inline(always)]
pub fn get_config_location() -> PathBuf {
    let username = username();
    let mut path = PathBuf::new();

    if cfg!(target_os = "linux") || cfg!(target_os = "freebsd")
       || cfg!(target_os = "openbsd") {
        if username == "root" {
            path.push("/root/.local/share");
        } else {
            path.push(&format!("/home/{}/.local/share", username));
        }
    } else if cfg!(target_os = "macos") {
        path.push(&format!("/Users/{}/", username));
    } else {
        path.push(format!("C:\\Users\\{}\\AppData\\", username));
    }
    
    path.push("mcget");
    
    std::fs::create_dir(&path).unwrap_or_default();
    std::fs::create_dir(&path.join("mods")).unwrap_or_default();
    std::fs::create_dir(&path.join("database")).unwrap_or_default();

    path
}