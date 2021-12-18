use {
    argh::{FromArgs},
    crate::{
        cli_helpers::*,
        api::prelude::*,
        config::*
    },
    indicatif::{ProgressBar, HumanBytes},

    std::{
        path::PathBuf,
    },
    tokio::{
        sync::mpsc::channel
    }
};

#[derive(FromArgs, Debug)]
#[argh(description = "Minecraft CurseForge package manager for mods")]
struct McGetArgs {
    #[argh(option, short = 's',
           description = "search mod in CurseForge registry")]
    pub search: Option<String>,

    #[argh(switch, short = 'a',
           description = "add first result to Modpack.yaml")]
    pub add: bool,

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

    if args.add {
        let first = results.first().unwrap();
        let mut pack = ModpackConfig::lookup();
        pack.modpack.mods.push(ModConfig{id: first.id});
        log("Saved mod:");
        print_mod(first);

        pack.store();
        return Ok(());
    }

    for modification in &results {
        print_mod(modification);
    }

    Ok(())
}

fn print_mod(modification: &Mod) {
    println!("{} (id: {}):\n\t{}\n\tMod loaders: {}\n\tCurseForge: {}", modification.name.red().bold(),
             modification.id,
             modification.summary.bold(), dump_mod_loaders(&modification), modification.curseforge);
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
    let mut handles = vec![];
    
    log("Getting files...");
    for modification in &pack.modpack.mods {
        let files = get_files(modification.id,
                                          &pack.modpack.version, &pack.modpack.mod_loader).await?;
        if files.is_empty() {
            log(&format!("Can't find mod {}", modification.id));
            std::process::exit(1);
        }

        let file = files.first().unwrap().clone();
        urls.push(file);
    }

    log(&format!("Downloading {} files...", urls.len().to_string().green()));
    let bar = ProgressBar::new(urls.len() as u64);
    bar.inc(0);
    
    let mut downloaded = 0usize;
    let mut downloaded_size = 0usize;
    let need_to_download = urls.len();
    
    let (tx, mut rx) = channel(32);

    for url in urls {
        let target = path.join(url.file_name);
        let txc = tx.clone();
        let downloader_task = tokio::spawn(async move {
            match download_to(&url.download_url, target.to_str().unwrap()).await {
                Ok(length) => {
                    txc.send(length).await.unwrap();
                    Ok(())
                },
                Err(e) => Err(e)
            }
        });

        handles.push(downloader_task);
    }

    while downloaded < need_to_download {
        let bytes = rx.recv().await.unwrap_or(0);
        if bytes == 0 {
            log("Failed to download files");
            std::process::exit(1);
        }
        bar.inc(1);

        downloaded += 1;
        downloaded_size += bytes;
    }
    bar.finish();
    
    log(&format!(
        "Downloaded size: {}",
        HumanBytes(downloaded_size as u64).to_string().bold()
    ));

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
