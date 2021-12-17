use {
    reqwest::{get},
    crate::{
        api::{
            objects::*
        },
        cli_helpers::*
    },
    urlencoding::encode
};

#[inline(always)]
pub fn get_url(path: &str, add_game: bool) -> String {
    let mut s = "https://addons-ecs.forgesvc.net/api/v2/addon/".to_string();
    s.push_str(path);
    if add_game {
        s.push_str("?gameId=432&sectionId=6&");
    }

    s
}

pub async fn get_files(id: usize, version: &str,
                       mod_loader: &str) -> RResult<Vec<ModFile>> {
    let mut path = get_url(&id.to_string(), false);
    path.push_str("/files");

    let res = get(
        path).await?.json::<Vec<ModFile>>().await?;
    let mut results = vec![];

    for result in &res {
        if result.contains_version(version) && result.contains_version(mod_loader) {
            results.push(result.clone());
        }
    }

    Ok(results)
}

pub async fn search_mod(query: &str, version: &Option<String>) -> RResult<Vec<Mod>> {
    let mut path = get_url("search", true);
    path.push_str("searchFilter=");
    path.push_str(&encode(query));

    if version.is_some() {
        path.push_str("&gameVersion=");
        path.push_str(&encode(&version.as_ref().unwrap()));
    }

    let res = get(path).await?
    .json::<Vec<Mod>>().await?;

    Ok(res)
}

