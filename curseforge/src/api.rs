use {
    crate::{
        models::*,
    },

    smallvec::SmallVec,
};

pub async fn get_files(
    id: i64,
    
    version: &Option<String>,
    loader: &Option<String>,
) -> Vec<ModFile> {
    let client = reqwest::Client::new();
    let files = client.get(&format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}/files", id))
                      .send()
                      .await
                      .expect("Failed to request files")
                      .json::<Vec<ModFile>>()
                      .await
                      .expect("Failed to parse files json");

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
