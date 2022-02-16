use {
    indicatif::*,
    std::{
        path::PathBuf,
        fs::File,
        io::Write,
    },

    tokio::{
        task::JoinHandle,
    },

    futures_util::StreamExt,
    colored::*,
};

#[derive(Debug)]
struct DownloadTarget {
    url: String,
    dest: String,
}

pub struct Downloader {
    urls: Vec<DownloadTarget>,
    parent: PathBuf,
}

// TODO: move it
async fn join_all(futs: &mut Vec<JoinHandle<()>>) {
    for fut in futs {
        fut.await
           .unwrap_or_default();
    }
}

impl Downloader {
    async fn download(
        url: String,
        dest: PathBuf,
        bar: ProgressBar
    ) {
        let request = reqwest::get(&url).await;
        if request.is_err() {
            println!("{} to download file {}", "Failed".red(), url);
            return;
        }
        let request = request.unwrap();
        let content_length = request.content_length().unwrap_or(209715200);
        bar.inc_length(content_length);

        let mut file = match File::create(&dest) {
            Ok(r) => r,
            Err(e) => {
                bar.println(&format!("{} to download file {}: {} (open error)", "Failed".red(), url, e));
                return;
            }
        };
        let mut stream = request.bytes_stream();
        
        bar.set_message(
            dest.file_name().unwrap().to_str().unwrap().to_owned()
        );

        while let Some(Ok(chunk)) = stream.next().await {
            match file.write_all(&chunk) {
                Ok(()) => {},
                Err(e) => {
                    std::fs::remove_file(&dest).unwrap_or_default();
                    bar.println(&format!("{} to download file {}: {} (write error)", "Failed".red(), url, e));
                    return;
                }
            }

            bar.inc(chunk.len() as u64);
        }
    }

    pub async fn download_all(self, max_workers: usize) {
        let mut multi = MultiProgress::new();
        let mut workers = vec![];

        for target in self.urls {
            if workers.len() >= max_workers {
                // TODO: copypasta
                tokio::task::spawn_blocking(
                    move || multi.join_and_clear()
                ).await.ok()
                       .unwrap_or(Ok(()))
                       .unwrap();
                join_all(&mut workers).await;

                multi = MultiProgress::new();
                workers.clear();
            }

            let progress = ProgressBar::new(0);
            let dest = self.parent.join(target.dest);

            progress.set_style(ProgressStyle::default_bar()
                    .template("[{elapsed_precise:5} - {eta:5}] {bar:40.cyan/blue} {bytes:7}/{total_bytes:7} {wide_msg}")
                    .progress_chars("##-"));
            multi.add(progress.clone());

            workers.push(tokio::spawn(async move {
                Self::download(
                    target.url,
                    dest,
                    progress
                ).await;
            }));
        }

        if !workers.is_empty() {
            // TODO: copypasta
            tokio::task::spawn_blocking(
                move || multi.join_and_clear()
            ).await.ok()
                   .unwrap_or(Ok(()))
                   .unwrap();
            join_all(&mut workers).await;
        }
    }

    pub fn new(
        parent_path: PathBuf,
    ) -> Self {
        Self{
            urls: Default::default(),
            parent: parent_path,
        }
    }

    pub fn extend(
        &mut self,
        urls: Vec<(String, String)>,
    ) {
        self.urls.extend(
            urls.into_iter().map(|v| DownloadTarget{ url: v.0, dest: v.1})
        )
    }
}
