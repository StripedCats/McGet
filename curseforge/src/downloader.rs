use {
    indicatif::{
        ProgressBar, HumanBytes
    },

    crate::{api::CurseForge, objects::*},
    std::path::Path,

    hyper::{
        Client, client::HttpConnector,
    },
    hyper_tls::HttpsConnector,
    
    tokio::{
        sync::mpsc::{
            channel
        }
    },
    colored::*
};

type ArcResult<T> = Result<T, Box<dyn std::error::Error + Send>>;

#[derive(Clone)]
pub struct DownloadTarget {
    pub id: usize,
    pub dest: String,
}

// Mass mod downloader
pub struct MassDownloader {
    pub progress: Option<ProgressBar>,

    files: Vec<DownloadTarget>,
}

impl MassDownloader {
    pub fn new() -> MassDownloader {
        MassDownloader{ progress: None,
                        files: Default::default() }
    }

    pub fn add_file(&mut self, id: usize, dest: String) {
        self.files.push(DownloadTarget{id, dest});
    }

    pub fn add_target(&mut self, target: DownloadTarget) {
        self.files.push(target);
    }

    async fn download_process(client: Client<HttpsConnector<HttpConnector>>,
                              file: &mut DownloadTarget, cf: CurseForge,
                              version: GameVersion) -> ArcResult<(usize, String)> {
        let mf = match cf.files(file.id, version).await {
            Ok(r) => r,
            Err(_) => { return Ok((0, file.id.to_string())); }
        };

        let latest = mf.latest();
        if latest.is_err() {
            return Ok((0, file.id.to_string()));
        }

        let mut url = latest.unwrap().download_url.clone();
        let dst_path = Path::new(&file.dest);

        file.dest = dst_path.join(&latest.unwrap().filename).to_str().unwrap().to_string();

        drop(mf);
        let response;

        loop {
            url = url.replace(" ", "%20");
            let loc = match client.get(url.parse().unwrap()).await {
                Ok(r) => r,
                Err(_) => { println!("Get"); return Ok((0, url.clone())); }
            };

            if loc.headers().contains_key("location") {
                url = loc.headers().get("location").unwrap().to_str().unwrap().to_string();
                continue;
            }

            response = loc;
            break;
        }

        let body = match hyper::body::to_bytes(response).await {
            Ok(r) => r,
            Err(_) => { println!("read"); return Ok((0, url.clone())); }
        };

        match std::fs::write(&file.dest, &body) {
            Ok(r) => r,
            Err(_) => { println!("write {}", file.dest); return Ok((0, url.clone())); }
        };
        
        Ok((body.len(), url))
    }

    pub async fn download(&mut self, cf: &CurseForge,
                          version: GameVersion) {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        self.progress = Option::Some(ProgressBar::new(self.files.len() as u64 + 1));

        let bar = self.progress.as_ref().unwrap();
        bar.inc(1);
        let (tx, mut rx) = channel(32);

        for file in &self.files {
            let tx = tx.clone();
            let mut file = file.clone();
            let client = client.clone();
            let cf = cf.clone();
            let version = version.clone();

            tokio::spawn(async move {
                match Self::download_process(client, &mut file, cf, version).await {
                    Ok((size, url)) => {
                        tx.send((size, url)).await.unwrap_or_default();
                    },

                    Err(_) => {
                        tx.send((0usize, file.id.to_string())).await.unwrap_or_default();
                    }
                }
            });
        }

        let mut downloaded = 0usize;
        let mut down_size = 0usize;
        while downloaded < self.files.len() {
            let (size, url) = rx.recv().await.unwrap();
            if size == 0 {
                bar.println(format!("Failed to download {}", url));
            } else {
                bar.println(format!("Successfully downloaded {} {}", HumanBytes(size as u64).to_string(), url));
            }

            bar.inc(1);
            bar.tick();
            down_size += size;
            downloaded += 1;
        }

        bar.finish();
        println!("{} downloaded {}", "Successfully".green(), HumanBytes(down_size as u64).to_string().bold());
    }
}
