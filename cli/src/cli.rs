use {
    argh::FromArgs,
    colored::*,
    
    curseforge::prelude::*,
    crate::{
        config::*, modpack::*
    }
};

#[derive(FromArgs)]
#[argh(description = "CurseForge package manager for Minecarft mods")]
pub struct CliApp {
    #[argh(option, short = 's',
           description = "search mods by query")]
    pub search: Option<String>,

    #[argh(option, short = 'v',
           description = "minecraft version")]
    pub version: Option<String>,

    #[argh(option, short = 'a',
           description = "add first match on search to modpack")]
    pub add: Option<String>,

    #[argh(option, short = 'l',
           description = "mod loader(e.g. forge)")]
    pub mod_loader: Option<String>,

    #[argh(option, description = "create modpack")]
    pub create_modpack: Option<String>,

    #[argh(option, short = 'd',
           description = "download and switch modpack")]
    pub download: Option<String>,

    #[argh(option, short = 's',
           description = "switch modpack")]
    pub switch: Option<String>,

    #[argh(option, short = 'r',
           description = "remove modpack")]
    pub remove: Option<String>
}

impl CliApp {
    #[inline(always)]
    fn dump_modinfo(mod_: &Mod) -> String {
        format!("{} (id: {}):\n\t{}\n\tMod loaders: {}\n\tCurseForge: {}", mod_.name.red(), 
                mod_.id, mod_.summary.bold(),
                Self::dump_mod_loaders(mod_),
                mod_.curseforge)
    }

    #[inline(always)]
    fn dump_mod_loaders(mod_: &Mod) -> String {
        if mod_.mod_loaders.is_none() {
            return "No information".to_string();
        }

        let mut out = String::new();
        for loader in mod_.mod_loaders.as_ref().unwrap() {
            out += &format!("\n\t  - {}", loader.green());
        }

        out
    }

    fn unwrap_or<T>(r: Option<T>, message: &str) -> T {
        match r {
            Some(s) => s,
            None => {
                println!("Error: {}", message.red());
                std::process::exit(1);
            }
        }
    }

    pub async fn search_fn(&self, cf: CurseForge) -> RResult<()> {
        println!("Searching for {}...", self.search.as_ref().unwrap().red());
        let results = cf.search(
            self.search.as_ref().unwrap(), self.version.as_ref()
        ).await?;
        if results.is_empty() {
            println!("Nothing was found");
            return Ok(());
        }

        if self.add.is_some() {
            let first = results.first().unwrap();
            let mut cfg = ModpackCfg::load(self.add.as_ref().unwrap());
            if cfg.mc.has_modid(first.id) {
                println!("Already had this one:");
                println!("{}", Self::dump_modinfo(first));
                return Ok(());
            }

            cfg.mc.mods.push(ModpackMod::new(first.id));
            cfg.store();

            println!("Found & added to modpack:");
            println!("{}", Self::dump_modinfo(first));

            return Ok(());
        }

        for result in results.iter().rev() {
            println!("{}", Self::dump_modinfo(&result));
        }

        Ok(())
    }

    pub async fn create_modpack_fn(&self) -> RResult<()> {
        let pack = ModpackCfg::new(self.create_modpack.as_ref().unwrap().clone(), Self::unwrap_or(
            self.version.as_ref(), "Game version is required for modpack creation").clone(), 
            self.mod_loader.as_ref().unwrap_or(&"forge".to_string()).clone(),
            self.create_modpack.as_ref().unwrap().clone() + ".yaml");
        pack.store();

        println!("Modpack {}.yaml is successfully created", self.create_modpack.as_ref().unwrap());

        Ok(())
    }

    pub async fn download_fn(&self, cf: CurseForge) -> RResult<()> {
        let pack = ModpackCfg::load(self.download.as_ref().unwrap());
        let version = GameVersion::new(pack.mc.version).with_loader(pack.mc.loader);
        let mut downloader = MassDownloader::new();
        let packs = get_config_location().join("modpacks").join(&pack.mc.name);
        std::fs::create_dir(packs.to_str().unwrap()).unwrap_or_default();

        println!("Resolving dependencies...");
        let deps = resolve_dependencies(&cf, pack.mc.mods.iter().map(
            |v| v.id
        ).collect(), version.clone(), packs.to_str().unwrap().to_string()).await;

        println!("Downloading mods...");
        for target in deps {
            downloader.add_target(target);
        }

        downloader.download(&cf, version).await;

        let cfg = McGetConfig::lookup();
        cfg.switch_modpack(&pack.mc.name);

        Ok(())
    }

    pub fn remove_modpack(&self) {
        let pack = self.remove.as_ref().unwrap();
        let cfg_dir = get_config_location().join("modpacks").join(pack);

        match std::fs::remove_dir_all(cfg_dir.to_str().unwrap()) {
            Ok(_) => {
                println!("{} Removed {}", "Successfully".green(),
                         cfg_dir.to_str().unwrap().bold());
            }

            Err(e) => {
                println!("Error: {}", e.to_string().red());
            }
        }
    }

    pub fn switch_modpack(&self) {
        let pack = self.switch.as_ref().unwrap();
        let cfg = McGetConfig::lookup();
        cfg.switch_modpack(pack);
    }

    pub async fn run() -> RResult<()> {
        let cf = CurseForge::new();
        let args: Self = argh::from_env();
        
        if args.switch.is_some() {
            args.switch_modpack();
        } else if args.remove.is_some() {
            args.remove_modpack();
        } else if args.download.is_some() {
            args.download_fn(cf).await?;
        } else if args.create_modpack.is_some() {
            args.create_modpack_fn().await?;
        } else if args.search.is_some() {
            args.search_fn(cf).await?;
        }

        Ok(())
    }
}
