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

fn unwrap_or<T, E>(result: Result<T, E>, message: &str) -> T {
    match result {
        Ok(r) => r,
        Err(_) => {
            log(&format!("{}: {}", "Failed to request some data".bold(), message));
            std::process::exit(1);
        }
    }
}

#[inline(always)]
pub fn get_url(path: &str, add_game: bool) -> String {
    let mut s = "https://addons-ecs.forgesvc.net/api/v2/addon/".to_string();
    s.push_str(path);
    if add_game {
        s.push_str("?gameId=432&sectionId=6&");
    }

    s
}

pub async fn download_to(file_url: &str, path: &str) -> RResult<usize> {
    let value = unwrap_or(
        get(file_url).await, &format!("Failed to download {}", file_url.red()),
    ).bytes().await;

    let data = unwrap_or(value, "Failed to recv bytes");
    std::fs::write(path, &data).unwrap();

    Ok(data.len())
}

pub async fn get_files(id: usize, version: &str,
                       mod_loader: &str) -> RResult<Vec<ModFile>> {
    let mut path = get_url(&id.to_string(), false);
    path.push_str("/files");

    let res = unwrap_or(
        get(path).await, &format!(
            "Failed to request files for ID#{}", id
        )).json::<Vec<ModFile>>().await.unwrap();

    let mut results = vec![];

    for result in &res {
        if result.contains_version(version) && result.contains_mod_loader(mod_loader) {
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

    let res = unwrap_or(get(path).await, &format!(
        "Error requesting mods by query {:?}", query
    )).json::<Vec<Mod>>().await;

    Ok(unwrap_or(res, &format!(
        "Error while parsing mods"
    )))
}

