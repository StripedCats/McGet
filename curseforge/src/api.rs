use {
    crate::{
        models::*,
    },

    tokio::{
        time::timeout,
    },

    std::time::Duration,
    smallvec::SmallVec,
    colored::*,
};

pub async fn get_files(
    id: i64,
    
    version: &Option<String>,
    loader: &Option<String>,

    timeout_secs: u64,
) -> Vec<ModFile> {
    let client = reqwest::Client::new();

    let mut retries = 0;
    let mut files = vec![];

    let req_timeout = Duration::from_secs(timeout_secs);

    while retries < 10 {
        retries += 1;
        let future_result = timeout(
            req_timeout,
            client.get(&format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}/files", id)).send()
        ).await;
        let response = match future_result {
            Ok(Ok(r)) => r,
            Ok(Err(e)) => {
                println!("Request error: {}, retrying... (retry no#{})", e, retries);
                continue;
            },

            Err(_) => {
                // println!("{} {} ({} seconds)", "[Error  ]".red(), "Timed out".red(), timeout_secs);
                continue;
            },
        };

        let ac_files =  response.json::<Vec<ModFile>>().await;
        if ac_files.is_err() {
            println!("Request error: {}, retrying... (retry no#{})", "Failed to parse files JSON".red(), retries);
            continue;
        }

        files = ac_files.unwrap();
        break;
    }

    if retries > 9 {
        println!("Failed to retrieve Mod#{} files ({} retries passed)", id, retries);
        std::process::exit(1);
    }

    files.into_iter()
         .filter(move |file| loader.is_none() || file.has_loader(loader.as_ref().unwrap()))
         .filter(move |file| version.is_none() || file.has_version(version.as_ref().unwrap()))
         .collect()
}

// TODO: Optimize this
pub async fn search_mod(
    name: &str,

    version: &Option<String>,
    loader: &Option<String>,
    reverse: bool
) -> Vec<ModEntry> {
    let mut query = {
        let mut vec = SmallVec::<[(&str, &str); 4]>::new();
        vec.insert_many(0, [
            ("gameId", "432"),
            ("sectionId", "6"),
            ("searchFilter", name)
        ].into_iter());

        vec
    };

    if version.is_some() {
        query.push(("gameVersion", version.as_ref().unwrap()));
    }

    let client = reqwest::Client::new();
    let mods = client.get("https://addons-ecs.forgesvc.net/api/v2/addon/search")
                     .query(&query[..])
                     .send()
                     .await
                     .expect("Failed to get mod")
                     .json::<Vec<ModEntry>>()
                     .await
                     .expect("Failed to parse Mod JSON");

    let iter = mods.into_iter()
                   .filter(move |entry| loader.is_none() || entry.mod_loaders.contains(loader.as_ref().unwrap()));
    
    if reverse {
        iter.rev().collect()
    } else {
        iter.collect()
    }
}
