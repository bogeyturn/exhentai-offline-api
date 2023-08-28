use crate::connections::Connections;
use crate::models::p_mixed::InsertThis;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

pub fn get_for_id(id: i32) {
    let mut conn = Connections::new(false);
    let ids = conn.get_p_mixed_service().get(id);
    let mut api_dump = conn.get_api_dump_service();
    for v in ids {
        let entry = api_dump.get(v).unwrap();
        let url = format!(
            "https://exhentai.org/g/{}/{}/",
            entry.gid,
            entry.token.unwrap()
        );
        println!("{}", url);
    }
}

pub fn run() {
    let mut conn = Connections::new(false);
    let mut h: HashMap<i32, Arc<Mutex<HashSet<i32>>>> = HashMap::new();
    //gid, first_gid, parent_gid, filecount, artist, group_name
    let items = conn.get_api_dump_service().all_ids();
    for (gid, first_gid, parent_gid, _, a, g) in items {
        let mut items = vec![gid];
        if let Some(fg) = first_gid {
            items.push(fg);
        }
        if let Some(pg) = parent_gid {
            items.push(pg);
        }
        if let Some(mut v) = process_hitomi(&mut conn, gid, a, g) {
            items.append(&mut v);
        }
        append_or_create(&mut h, &items);
    }
    let h = h
        .into_iter()
        .map(|v| InsertThis {
            gid: v.0,
            p: v.1.lock().unwrap().clone().into_iter().min().unwrap(),
        })
        .collect::<Vec<_>>();
    let c = h.chunks(1000);
    for chunk in c {
        conn.get_p_mixed_service().insert(chunk)
    }
}

fn process_hitomi(
    conn: &mut Connections,
    id: i32,
    other_artists: Option<String>,
    other_groups: Option<String>,
) -> Option<Vec<i32>> {
    Some(
        match hitomi_matches(conn, id, &other_artists, &other_groups) {
            Some(v) => v,
            None => return None,
        }
        .into_iter()
        .filter(|v| hitomi_matches(conn, *v, &other_artists, &other_groups).is_some())
        .collect::<Vec<_>>(),
    )
}

#[derive(Serialize, Deserialize)]
struct HitomiArtist {
    #[serde(alias = "group", alias = "artist")]
    v: String,
    url: Option<String>,
}

fn hitomi_matches(
    conn: &mut Connections,
    id: i32,
    other_artists: &Option<String>,
    other_groups: &Option<String>,
) -> Option<Vec<i32>> {
    let (_, langs, artists, groups) = match conn.get_hitomi_service().get(id) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let ac = compare_artists_or_groups(other_artists, &artists);
    let gc = compare_artists_or_groups(other_groups, &groups);
    if (ac == State::DoesNotExist && gc == State::DoesNotExist)
        || (ac == State::DoesNotExist && gc == State::Unknown)
        || (ac == State::Unknown && gc == State::DoesNotExist)
    {
        return None;
    }
    let mut res = vec![];
    if let Some(langs) = langs {
        let parsed: Vec<Value> = serde_json::from_str(&langs).unwrap();
        for item in parsed {
            let id = match &item["galleryid"] {
                Value::Null => panic!("is null"),
                Value::Bool(v) => *v as i32,
                Value::Number(v) => v.as_i64().unwrap() as i32,
                Value::String(v) => v.parse().unwrap(),
                Value::Array(_) => panic!("is Array"),
                Value::Object(_) => panic!("is Object"),
            };
            res.push(id);
        }
    }
    Some(res)
}

#[derive(Eq, PartialEq)]
enum State {
    Appears,
    Unknown,
    DoesNotExist,
}

fn compare_artists_or_groups(item1: &Option<String>, hitomi_item: &Option<String>) -> State {
    if item1.is_none() && hitomi_item.is_none() {
        return State::Unknown;
    }
    let parsed1: Vec<String> = match item1 {
        None => return State::Unknown,
        Some(v) => serde_json::from_str(&v.replace('\'', "\"")).unwrap(),
    };

    let hitomi_parsed: Vec<HitomiArtist> = match hitomi_item {
        None => return State::Unknown,
        Some(v) => serde_json::from_str(v).unwrap(),
    };
    if parsed1.is_empty() || hitomi_parsed.is_empty() {
        return State::Unknown;
    }
    let processed1 = parsed1
        .iter()
        .map(|v| v.to_ascii_lowercase())
        .collect::<Vec<_>>();
    let processed2 = hitomi_parsed
        .iter()
        .map(|v| v.v.to_ascii_lowercase())
        .collect::<Vec<_>>();
    for item in processed1.iter() {
        if processed2.contains(item) {
            return State::Appears;
        }
    }
    for item in processed2.iter() {
        if processed1.contains(item) {
            return State::Appears;
        }
    }
    State::DoesNotExist
}

fn append_or_create(h: &mut HashMap<i32, Arc<Mutex<HashSet<i32>>>>, items: &[i32]) {
    let v = items.iter().map(|v| (*v, h.get(v)));
    let ids = v.clone().map(|v| v.0).collect::<Vec<_>>();
    let mut filtered = v
        .filter_map(|v| v.1)
        .map(|v| (Arc::as_ptr(v), v))
        .collect::<HashMap<_, _>>()
        .into_iter()
        .map(|v| v.1)
        .collect::<Vec<_>>();
    let arc: Arc<Mutex<HashSet<i32>>> = if filtered.is_empty() {
        Arc::new(Mutex::new(HashSet::new()))
    } else if filtered.len() == 1 {
        filtered.remove(0).clone()
    } else {
        let item = filtered.remove(0).clone();
        {
            let mut item = item.lock().unwrap();
            for other in filtered {
                for o in other.lock().unwrap().iter() {
                    item.insert(*o);
                }
            }
        }

        item
    };
    {
        let mut arc = arc.lock().unwrap();

        for item in items {
            arc.insert(*item);
        }
    }
    for id in ids {
        h.insert(id, arc.clone());
    }
}
