use {
    argh::{FromArgs},
    crate::{
        cli_helpers::*,
        api::prelude::*,
        config::*
    },

    std::{
        path::PathBuf
    }
};

#[derive(FromArgs, Debug)]
#[argh(description = "Minecraft CurseForge package manager for mods")]
struct McGetArgs {
    #[argh(option, short = 's',
           description = "search mod in CurseForge registry")]
    pub search: Option<String>,

    #[argh(option, short = 'v',
           description = "version of minecraft")]
    pub version: Option<String>,

    #[argh(switch, short = 'd',
           description = "download modpack from file Modpack.yaml")]
    pub download: bool,

    #[argh(option, short = 'm',
           description = "init modpack directory")]
    pub init_modpack: Option<String>
}

fn dump_mod_loaders(modification: &Mod) -> String {
    match &modification.mod_loaders {
        Some(loaders) => {
            let mut string = String::new();
            for loader in loaders {
                string.push_str(&format!("\n\t  - {}", loader.as_str()));
            }

            string
        },
        None => "No information".to_string()
    }
}

async fn search(args: McGetArgs) -> RResult<()> {
    log(&format!("Searching for {}...", args.search.as_ref().unwrap().green().bold()));
    let results = {
        let _indicator = SingleProgressIndicator::new();

        search_mod(&args.search.unwrap(), &args.version).await?
    };

    for modification in &results {
        println!("{} (id: {}):\n\t{}\n\tMod loaders: {}\n\tCurseForge: {}", modification.name.red().bold(),
                 modification.id,
                 modification.summary.bold(), dump_mod_loaders(&modification), modification.curseforge);
    }

    Ok(())
}

async fn download(args: McGetArgs) -> RResult<()> {
    let pack = ModpackConfig::lookup();
    std::fs::create_dir(&pack.modpack.name).unwrap_or_default();
    if !std::path::Path::new(&pack.modpack.name).exists() {
        log(&format!(
            "Can't create directory {}",
            pack.modpack.name.bold()
        ));
        std::process::exit(1);
    }

    let path = PathBuf::from(&pack.modpack.name);
    let mut urls = vec![];
    
    for modification in &pack.modpack.mods {
        let files = get_files(modification.id,
                                          &pack.modpack.version, &pack.modpack.mod_loader).await?;
        if files.is_empty() {
            log(&format!("Can't find mod {}", modification.id));
            std::process::exit(1);
        }

        let file = files.first().unwrap().clone();
        log(&format!("Downloading {}...", file.download_url.bold()));
        urls.push(file);
    }

    Ok(())
}

pub async fn start() -> RResult<()> {
    let args: McGetArgs = argh::from_env();

    if args.download {
        download(args).await?;
    } else if args.init_modpack.is_some() {
        let modpack = ModpackConfig::default()
                                  .with_name(args.init_modpack.unwrap());
        modpack.store();
    } else if args.search.is_some() {
        search(args).await?;
    }

    Ok(())
}
