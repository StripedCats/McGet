use {
    crate::{
        models::*,
    },
};

// TODO: add version & mod loader to query
pub async fn search_mod(
    name: &str,
) -> Vec<ModEntry> {
    // let mut path = String::from("https://addons-ecs.forgesvc.net/api/v2/addon/search?gameId=432&sectionId=6&searchFilter=thaumcraft");
    let client = reqwest::Client::new();
    let mods = client.get("https://addons-ecs.forgesvc.net/api/v2/addon/search")
                     .query(&[
                         ("gameId", "432"),
                         ("sectionId", "6"),
                         ("searchFilter", name)
                         ])
                     .send()
                     .await
                     .expect("Failed to get mod")
                     .json::<Vec<ModEntry>>()
                     .await
                     .expect("Failed to parse Mod JSON");

    mods
}
