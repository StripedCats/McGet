use {
    crate::{
        config::*,
        
        local_pack::*,

        fmt::*,
    },

    clap::*,
    colored::*,
    curseforge::prelude::*,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Search mods in CurseForge Minecraft repository
    Search {
        /// CurseForge mod query
        #[clap(short, long)]
        query: String,

        /// By default McGet will print mods in reverse order for better readability, but
        /// you can use direct order
        #[clap(short, long)]
        direct_order: bool,

        /// Mod loader (currently supported forge, fabric, liteloader)
        #[clap(short, long)]
        loader: Option<String>,

        /// Minecraft version
        #[clap(short, long)]
        version: Option<String>,

        /// Add `n` mods from result
        #[clap(short, long, default_value_t = 0)]
        add_n: u64,

        /// Add all mods from result
        #[clap(long)]
        add_all: bool,

        /// Add selected mods to specified modpack
        #[clap(short, long)]
        to: Option<String>,
    },

    /// Download modpack and switch to it
    Download {
        /// Modpack name (file name with .yaml extension or name without it)
        #[clap(short, long)]
        #[clap(alias = "modpack")]
        #[clap(alias = "file")]
        pack: String,

        /// Download workers
        #[clap(short, long, default_value_t = 4)]
        workers: u8,

        /// Dependency resolver timeout
        #[clap(short, long, default_value_t = 5)]
        timeout: u64,
    },

    /// Create YAML modpack file
    Create {
        /// Modpack name
        #[clap(short, long)]
        name: String,

        /// Modpack Minecraft version
        #[clap(short, long)]
        version: String,

        /// Case insensitive mod loader (e.g. forge, fabric, liteloader)
        #[clap(short, long)]
        #[clap(alias = "mod-loader")]
        loader: Option<String>,
    },

    /// Switch to modpack (make symbolic link to Minecraft mods)
    Switch {
        /// Local modpack name
        #[clap(short, long)]
        name: String,
    },

    /// Remove Modpack
    Remove {
        /// Local modpack name
        #[clap(short, long)]
        name: String,
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Search{
            query,
            direct_order,
            loader,
            version,
            add_n,
            add_all,
            to
        } => {
            println!("Querying mod {}...", query.green());

            let results = search_mod(
                query,
                version,
                loader,
                !direct_order
            ).await;
            for result in results.iter() {
                println!("{}", format_mod(result));
            }

            if add_n != &0 || add_all != &false {
                if to.is_none() {
                    println!("{}: --to keyword is required to add mods", "Failed".red());
                    return;
                }

                let take_n;
                if add_n != &0 {
                    take_n = *add_n;
                } else {
                    take_n = u64::MAX;
                }

                let dest = to_yaml_ext(to.as_ref().unwrap());
                let mut pack = LocalModPack::load_file(&dest);

                for (mut index, mut result) in results.iter().enumerate() {
                    if index as u64 >= take_n {
                        break;
                    } else if !direct_order {
                        index = results.len() - index - 1;
                        result = &results[index];
                    }

                    if pack.by_id(result.id).is_some() {
                        println!("{} is already in modpack, skipping...", result.name.red());
                        continue;
                    }

                    pack.mods.push(
                        LocalMod::CurseForge(CurseForgeSource{
                            id: result.id,
                            date: None,
                            filename: None,
                        })
                    );

                    println!("{} added {} to {}", "Successfully".green(), result.name.green(), dest);
                }

                if take_n == u64::MAX {
                    println!("{} added {} mods to {}", "Successfully".green(), results.len(), dest);
                }

                pack.store_to_file(&dest);
            }
        },

        Commands::Download{
            pack,
            workers,
            timeout,
        } => {
            let file = to_yaml_ext(pack);
            let pack = LocalModPack::load_file(&file);
            
            let deps = spawn_resolvers(
                pack.mods.iter()
                    .filter(|mod_| mod_.is_curseforge())
                    .map(|v| v.to_curseforge_source())
                    .map(|v| v.id)
                    .collect(),
                *workers as usize,
                Some(pack.version),
                Some(pack.loader),
                *timeout
            ).await;

            println!("{:#?}", deps);
        },

        Commands::Create{
            name,
            version,
            loader
        } => {
            let n_loader: String;
            if let Some(l) = loader.as_ref() {
                n_loader = l.clone();
            } else {
                n_loader = "Forge".to_owned();
            }

            let dest = to_yaml_ext(name);
            let loader = n_loader;
            let pack = LocalModPack::new(
                name.clone(),
                version.clone(),
                loader
            );

            pack.store_to_file(&dest);
            println!("Stored to {}", dest.green());
        },

        x => {
            eprintln!("[FIXME] Unmatched {:?}", x);
        }
    }
}

mod fmt;
mod config;
mod local_pack;
