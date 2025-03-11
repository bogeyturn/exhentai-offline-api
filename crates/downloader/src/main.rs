use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use reqwest::{header::USER_AGENT, Client, Proxy};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::read_dir;
use tokio::{sync::Mutex, task, time::sleep};

fn chunk_vec<T>(items: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    let mut iter = items.into_iter();
    let mut chunks = Vec::new();
    loop {
        let chunk = iter.by_ref().take(chunk_size).collect::<Vec<_>>();
        if chunk.is_empty() {
            break;
        }
        chunks.push(chunk);
    }

    chunks
}
#[derive(Deserialize, Serialize)]
struct Temp2 {
    gmetadata: Vec<Kind>,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Kind {
    Err { gid: u64, error: String },
    Success(Gmetadata),
}

#[derive(Deserialize, Serialize)]
struct Gmetadata {
    gid: u64,
    token: String,
    current_gid: Option<String>,
    parent_gid: Option<String>,
    first_gid: Option<String>,
    current_key: Option<String>,
    first_key: Option<String>,
    parent_key: Option<String>,
    title: String,
    title_jpn: Option<String>,
    tags: Vec<String>,
    dumped: u64,
    rating: String,
    category: String,
    filecount: String,
    expunged: bool,
    torrents: Value,
    uploader: String,
    thumb: String,
    posted: String,
    torrentcount: String,
    filesize: u64,
}

fn merge(path: PathBuf) -> Vec<HashMap<String, Value>> {
    if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
        return vec![];
    }
    let data: Temp2 =
        serde_json::from_reader(File::open(&path).unwrap()).expect(&format!("{}", path.display()));
    let downloaded: u64 = path
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or_default()
        .split_once("-")
        .unwrap()
        .0
        .parse()
        .unwrap();
    data.gmetadata
        .into_iter()
        .map(|mut v| {
            //v.insert("dumped".to_owned(), json!(downloaded));
            v
        })
        .collect::<Vec<_>>();
    todo!()
}

use rand::{distributions::Alphanumeric, Rng};

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn filter(v: Vec<(i32, String)>) -> Vec<(i32, String)> {
    println!("{}", v.len());
    let paths = read_dir("infos")
        .map(|v| {
            v.filter_map(|v| v.ok())
                .map(|v| v.path())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let ids = paths.into_iter().flat_map(get_ids).collect::<HashSet<_>>();
    let items = v
        .into_iter()
        .filter(|v| ids.get(&(v.0 as u64)).is_none())
        .collect::<Vec<_>>();
    println!("{}", items.len());
    items
}
#[derive(Deserialize)]
struct Temp {
    gmetadata: Vec<Id>,
}

#[derive(Deserialize)]
struct Id {
    gid: u64,
}
fn get_ids(v: PathBuf) -> Vec<u64> {
    let items: Result<Temp, _> = serde_json::from_reader(File::open(v).unwrap());
    items
        .map(|v| v.gmetadata.into_iter().map(|v| v.gid).collect::<Vec<_>>())
        .unwrap_or_default()
}

fn merge_files() {
    let items = read_dir("infos")
        .unwrap()
        .filter_map(|v| v.ok())
        .map(|v| v.path())
        .flat_map(merge)
        .collect::<Vec<_>>();
    File::create("merge")
        .unwrap()
        .write_all(
            serde_json::to_string(&Temp2 { gmetadata: vec![] })
                .unwrap()
                .as_bytes(),
        )
        .unwrap()
}

#[tokio::main]
async fn main() {
    let mut conn = models::establish_connection_pg(
        "postgres://postgres:password@localhost:5432/offline_doujinshi_api",
    );
    let items = Arc::new(Mutex::new(chunk_vec(filter(vec![]), 25)));
    let mut handles = vec![];

    let num_threads = 5;
    let proxy = true;

    const AGENTS:[&'static str; 10] = ["Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 Edg/123.0.2420.81",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 OPR/109.0.0.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14.4; rv:124.0) Gecko/20100101 Firefox/124.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_4_1) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Safari/605.1.15",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_4_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 OPR/109.0.0.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
    "Mozilla/5.0 (X11; Linux i686; rv:124.0) Gecko/20100101 Firefox/124.0"];
    //curl -s "https://api.nordvpn.com/v1/servers?limit=0" | jq '[.[] | select(.status == "online" and (.technologies | .[].identifier == "socks")) | .hostname]'
    let mut servers: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    for i in 0..num_threads {
        let items_clone = Arc::clone(&items);
        let agent = AGENTS[i % 10];
        let servers = servers.clone();
        handles.push(task::spawn(async move {
            let new_proxy = ||async {
                let proxy_url = servers.lock().await.pop()?;
                Some(Client::builder()
            .proxy(
                Proxy::all(format!("socks5://username:password@{proxy_url}:1080")).unwrap(),
            )
            .build()
            .unwrap())
            };
            let mut client = match proxy {
                false => Client::new(),
                true => new_proxy().await.unwrap(),
            };
            while let Some(v) = {
                let mut items = items_clone.lock().await;
                items.pop()
            } {
                loop {
                    let data = client
                        .post("http://e-hentai.org/api.php")
                        .header(USER_AGENT, agent)
                        .json(&json!({ "method": "gdata","namespace": 1,"gidlist": v}))
                        .send()
                        .await;
                    match data {
                        Ok(v) => {
                            if v.status().is_success() {
                                let text = v.text().await.unwrap();
                                if text.contains("This IP address has been temporarily banned due to an excessive request rate.") {
                                    client = match new_proxy().await {
                                        Some(v) => v,
                                        None => break
                                    };
                                    continue;
                                }
                                let fname = format!(
                                    "infos/{}-{}",
                                    SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_millis(),
                                    generate_random_string(8)
                                );
                                File::create(fname)
                                    .unwrap()
                                    .write_all(text.as_bytes())
                                    .unwrap();
                                sleep(Duration::from_millis(400)).await;

                                break;
                            } else {
                                println!("Error2");
                                sleep(Duration::from_secs(6)).await
                            }
                        }
                        Err(_) => {
                            println!("Error");
                            sleep(Duration::from_secs(6)).await
                        }
                    }
                }
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
