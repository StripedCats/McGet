use {
    hyper::{
        Client, client::HttpConnector,
        Response
    },
    urlencoding::encode,
    hyper_tls::HttpsConnector,

    crate::{
        objects::*
    }
};

pub type RResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Clone)]
pub struct CurseForge {
    client: Client<HttpsConnector<HttpConnector>>
}

async fn web_request(http: &Client<HttpsConnector<HttpConnector>>, url: &str) -> RResult<Response<hyper::Body>> {
    let mut retries = 10usize;
    let mut response = None;
    while retries > 0 {
        response = Option::Some(http.get(url.parse().unwrap()).await?);
        if response.as_ref().unwrap().status() == 200 {
            break;
        }
        println!("Retrying...");

        retries -= 1;
    }
    if response.is_none() {
        println!("Web request failed");
        std::process::exit(1);
    }

    Ok( response.unwrap() )
}

impl CurseForge {
    pub async fn search(&self, query: &str, version: Option<&String>) -> RResult<Vec<Mod>> {
        let mut url = Self::root_url("search", true);
        url.push_str("searchFilter=");
        url.push_str((&encode(query)).as_ref());

        if version.is_some() {
            url.push_str("&gameVersion=");
            url.push_str((&encode(version.unwrap())).as_ref());
        }

        let response = web_request(&self.client, &url).await?;
        let b_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body = std::str::from_utf8(&b_bytes).unwrap();

        Ok( serde_json::from_str(body).unwrap() )
    }

    pub async fn files(&self, id: usize, game: GameVersion) -> RResult<Vec<ModFile>> {
        let mut url = Self::root_url(&id.to_string(), false);
        url.push_str("/files");

        let response = web_request(&self.client, &url).await?;
        let b_bytes = hyper::body::to_bytes(response.into_body()).await?;
        let body = std::str::from_utf8(&b_bytes).unwrap();

        let resp: Vec<ModFile> = serde_json::from_str(body).unwrap();

        Ok( resp.where_mods(&game.version, game.mod_loader.as_ref()) )
    }

    #[inline(always)]
    fn root_url(path: &str, add_game: bool) -> String {
        let mut url = "https://addons-ecs.forgesvc.net/api/v2/addon/".to_string();
        url.push_str(path);

        if add_game {
            url.push_str("?gameId=432&sectionId=6&");
        }

        url
    }

    pub fn new() -> CurseForge {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        Self{client}
    }
}
