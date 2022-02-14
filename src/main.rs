use {
    crate::{
        config::*,
        pack::*,
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
        loader: String,
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
            for result in results {
                println!("{}", format_mod(&result));
            }
        },

        x => {
            eprintln!("[FIXME] Unmatched {:?}", x);
        }
    }
}

mod config;
mod pack;
mod fmt;
