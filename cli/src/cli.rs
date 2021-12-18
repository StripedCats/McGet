use {
    argh::FromArgs,
    colored::*,
    
    curseforge::prelude::*
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

    #[argh(option, description = "create modpack")]
    pub create_modpack: Option<String>
}

impl CliApp {
    pub async fn search_fn(&self, cf: CurseForge) -> RResult<()> {
        println!("Searching for {}...", self.search.as_ref().unwrap().red());
        let results = cf.search(
            self.search.as_ref().unwrap(), self.version.as_ref()
        ).await?;

        for result in results {
            println!("{}", Self::dump_modinfo(&result));
        }

        Ok(())
    }

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

    pub async fn run() -> RResult<()> {
        let cf = CurseForge::new();
        let args: Self = argh::from_env();
        
        if args.search.is_some() {
            args.search_fn(cf).await?;
        }

        Ok(())
    }
}
