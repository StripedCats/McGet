use {
    tokio::{
        sync::mpsc::{
            channel, Sender
        }
    },

    crate::{api::*, objects::*,
            downloader::DownloadTarget},
    std::{
        collections::HashSet,
        sync::Arc
    },
    colored::*
};

#[async_recursion::async_recursion]
async fn resolve_dependency(of: usize, cf: Arc<CurseForge>,
                            deps: Sender<usize>, version: GameVersion,
                            root: bool) {
    let files;
    match cf.files(of, version.clone()).await {
        Ok(r) => { files = Option::Some(r); },
        Err(_) => {
            println!("{} to resolve dependency of modid {}", "Failed".red(), of);
            files = None;
        }
    };

    if files.is_none() || (files.is_some() && files.as_ref().unwrap().latest().is_err()) {
        if root {
            deps.send(0).await.unwrap_or_default();
        }

        return;
    }

    let latest = files.as_ref().unwrap().latest().unwrap();

    let mut tasks = vec![];
    for dep in &latest.dependencies {
        let version = version.clone();
        let addon_id = dep.addon_id;
        let cf = cf.clone();
        let rdeps = deps.clone();

        tasks.push(tokio::spawn(async move {
            resolve_dependency(addon_id, cf, rdeps, version, false).await;
        }));
        deps.send(dep.addon_id).await.unwrap_or_default();
    }

    for task in tasks {
        task.await.unwrap_or_default();
    }

    if root {
        deps.send(0).await.unwrap_or_default();
    }
}

pub async fn resolve_dependencies(cf: &CurseForge, of: Vec<usize>,
                                  game: GameVersion, path: String) -> Vec<DownloadTarget> {
    let mut targets = vec![];
    let mut dependency_map: HashSet<usize> = Default::default();
    let (tx, mut rx) = channel(32);
    let cf = Arc::new(cf.clone());
    let mut tasks = vec![];

    for dep in &of {
        let tx = tx.clone();
        let cf = cf.clone();
        let ver = game.clone();
        let id = dep.clone();

        tx.send(id).await.unwrap_or_default();
        tasks.push(tokio::spawn(async move {
            resolve_dependency(id, cf, tx, ver, true).await;
        }));
    }

    let mut resolved = 0usize;
    while resolved < tasks.len() {
        let modid = rx.recv().await.unwrap();
        if modid == 0 { resolved += 1; continue; }
        if !dependency_map.contains(&modid) {
            targets.push(DownloadTarget{id: modid, dest: path.clone()});
            dependency_map.insert(modid);

            println!(">> Resolved dependency {}", modid);
        }
    }

    targets
}