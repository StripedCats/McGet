use {
    crate::{
        api::*,
        models::*,
        resolver_meta::*,
    },

    tokio::{
        sync::Mutex,
        task::JoinHandle,
    },

    std::{
        sync::Arc,
    },

    rustc_hash::FxHashMap,
    async_recursion::async_recursion,
    colored::*,
};

// Resolve dependencies of a signle mod
#[async_recursion]
pub async fn resolve_dependencies(
    id: i64,
    dep_of: i64,
    dep_of_str: String,

    version: Option<String>,
    loader: Option<String>,

    dep_db: Arc<Mutex<FxHashMap<i64, LocalDependency>>>,
    timeout: u64,
) {
    {
        let mut lock = dep_db.lock().await;
        if lock.insert(id, LocalDependency{ id, url: "".to_owned() }).is_some() {
            return;
        }
    }

    let files = get_files(
        id,
        &version,
        &loader,
        timeout,
    ).await;

    if files.is_empty() {
        println!("{} to resolve dependency of Mod#{}", "[Error]".red(), id);
        return;
    } else if id != dep_of {
        println!(
            "{} Resolved {} dependency of {}",
            "[Ok     ]".green(),
            files.last()
                 .as_ref()
                 .unwrap()
                 .name,
            dep_of_str,
        );
    } else {
        println!("{} Resolving dependencies of {}...", "[Initial]".blue(), files.last().as_ref().unwrap().name);
    }

    let latest = latest_file(files);
    {
        let mut lock = dep_db.lock().await;
        let dep = lock.get_mut(&id).unwrap();
        dep.url = latest.download_url;
    }

    for dependency in latest.dependencies
                            .iter()
                            .filter(|m| m.dep_type == 3) {
        resolve_dependencies(
            dependency.id,
            id,
            latest.name.clone(),
            version.clone(),
            loader.clone(),
            dep_db.clone(),
            timeout,
        ).await;
    }
    
    if id == dep_of {
        println!("{} Resolved all dependencies for {}!", "[Success]".bright_green(), latest.name);
    }
}

#[inline]
fn latest_file(mods: Vec<ModFile>) -> ModFile {
    mods.into_iter().max_by(|a, b| {
        a.date.cmp(&b.date)
    }).unwrap()
}

async fn join_all(futs: &mut Vec<JoinHandle<()>>) {
    for fut in futs {
        fut.await
           .unwrap_or_default();
    }
}

pub async fn spawn_resolvers(
    ids: Vec<i64>,
    workers_number: usize,

    version: Option<String>,
    loader: Option<String>,

    timeout: u64,
) -> FxHashMap<i64, LocalDependency> {
    let mut workers = vec![];
    let dep_db = Arc::new(
        Mutex::new(
            Default::default()
        )
    );

    for (index, id) in ids.iter().cloned().enumerate() {
        if index >= workers_number {
            join_all(&mut workers).await;
            workers.clear();
        }

        let deps = dep_db.clone();
        let version = version.clone();
        let loader = loader.clone();

        workers.push(tokio::spawn(async move {
            resolve_dependencies(
                id,
                id,
                "".to_owned(),
                version,
                loader,
                deps,
                timeout,
            ).await;
        }));
    }

    if !workers.is_empty() {
        join_all(&mut workers).await;
    }

    Arc::try_unwrap(dep_db).unwrap().into_inner()
}
