use std::collections::{HashMap, HashSet};

use models::models::api::{ExGallery, FailedSerice};
use models::models::{api_dump, failed};
use models::{
    add_characters, add_entry, add_failed, add_languages, add_parodies, add_tags, add_temp_tags,
    add_users, establish_connection_pg, establish_connection_sqlite, Category,
};

fn main() {
    let mut v = establish_connection_sqlite("./dbs/api_dump.sqlite");
    let mut failed = establish_connection_sqlite("./dbs/failed.sqlite");

    let pg_conn = &mut establish_connection_pg(
        "postgres://postgres:password@localhost:5432/offline_doujinshi_api",
    );
    let failed = failed::get_all(&mut failed)
        .into_iter()
        .map(|v| (v.gid, v.reason))
        .collect::<HashMap<_, _>>();
    let v = api_dump::all(&mut v);
    let mut users = HashSet::new();
    let mut parodies = HashSet::new();
    let mut characters = HashSet::new();
    let mut tags = HashSet::new();
    let mut temp_tags = HashSet::new();
    let mut languages = HashSet::new();

    for item in &v {
        if let Some(uploader) = item.uploader.clone() {
            users.insert(uploader);
        }
        users.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.artist.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        users.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.group.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        parodies.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.parody.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        characters.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.character.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        tags.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.female.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        tags.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.male.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        tags.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.mixed.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        tags.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.other.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        temp_tags.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.rest.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        users.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.cosplayer.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
        languages.extend(
            serde_json::from_str::<Vec<String>>(&convert_quotes(
                &item.language.clone().unwrap_or("[]".to_string()),
            ))
            .unwrap(),
        );
    }

    let users = add_users(pg_conn, users)
        .into_iter()
        .map(|v| (v.name, v.id))
        .collect::<HashMap<_, _>>();
    let parodies = add_parodies(pg_conn, parodies)
        .into_iter()
        .map(|v| (v.name, v.id))
        .collect::<HashMap<_, _>>();
    let characters = add_characters(pg_conn, characters)
        .into_iter()
        .map(|v| (v.name, v.id))
        .collect::<HashMap<_, _>>();
    let tags = add_tags(pg_conn, tags)
        .into_iter()
        .map(|v| (v.name, v.id))
        .collect::<HashMap<_, _>>();
    let temp_tags = add_temp_tags(pg_conn, temp_tags)
        .into_iter()
        .map(|v| (v.name, v.id))
        .collect::<HashMap<_, _>>();
    let languages = add_languages(pg_conn, languages)
        .into_iter()
        .map(|v| (v.language, v.id))
        .collect::<HashMap<_, _>>();
    let mut failed = add_failed(pg_conn, failed);
    let tag_to_id = |s: String| *tags.get(&s.replace("_", " ")).unwrap();
    let temp_tag_to_id = |s: String| *temp_tags.get(&s.replace("_", " ")).unwrap();

    let user_to_id = |s| *users.get(&s).unwrap();
    let language_to_id = |s| *languages.get(&s).unwrap();
    let parody_to_id = |s| *parodies.get(&s).unwrap();
    let character_to_id = |s| *characters.get(&s).unwrap();
    for item in v {
        let artists = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.artist.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(user_to_id(s)))
        .collect::<Vec<_>>();
        let groups = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.group.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(user_to_id(s)))
        .collect::<Vec<_>>();
        let parodies = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.parody.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(parody_to_id(s)))
        .collect::<Vec<_>>();
        let characters = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.character.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(character_to_id(s)))
        .collect::<Vec<_>>();
        let female = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.female.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(tag_to_id(s)))
        .collect::<Vec<_>>();
        let male = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.male.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(tag_to_id(s)))
        .collect::<Vec<_>>();
        let mixed = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.mixed.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(tag_to_id(s)))
        .collect::<Vec<_>>();
        let other = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.other.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(tag_to_id(s)))
        .collect::<Vec<_>>();
        let rest = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.rest.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(temp_tag_to_id(s)))
        .collect::<Vec<_>>();
        let cosplayers = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.cosplayer.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(user_to_id(s)))
        .collect::<Vec<_>>();
        let languages = serde_json::from_str::<Vec<String>>(&convert_quotes(
            &item.language.unwrap_or("[]".to_string()),
        ))
        .unwrap()
        .into_iter()
        .map(|s| Some(language_to_id(s)))
        .collect::<Vec<_>>();
        let item = ExGallery {
            gid: item.gid,
            title: item.title.unwrap(),
            title_jpn: item.title_jpn,
            token: item.token.unwrap(),
            category: Category::from(item.category.unwrap().as_str()) as i32,
            uploader: item.uploader.map(user_to_id),
            posted: item.posted.unwrap(),
            thumb: item.thumb.unwrap(),
            filesize: item.filesize.unwrap(),
            filecount: item.filecount.unwrap(),
            expunged: match item.expunged.unwrap() {
                0 => false,
                1 => true,
                _ => unreachable!(),
            },
            torrentcount: item.torrentcount.unwrap(),
            torrents: item.torrents.unwrap_or("[]".to_owned()),
            rating: (item.rating.unwrap() as f64 * 1_000_000.0).round() / 1_000_000.0,
            artists,
            groups,
            parodies,
            characters,
            female,
            male,
            languages,
            mixed,
            other,
            cosplayers,
            rest,
            parent_gid: item.parent_gid,
            first_gid: item.first_gid,
            disowned: match item.disowned.unwrap_or_default() {
                0 => false,
                1 => true,
                _ => unreachable!(),
            },
            removed: match item.removed.unwrap() {
                0 => None,
                1 => Some(failed.remove(&item.gid).unwrap_or(100)),
                _ => unreachable!(),
            },
            dumped: item.dumped,
        };
        add_entry(pg_conn, item);
    }
    FailedSerice { conn: pg_conn }.add(failed).unwrap()
}

fn convert_quotes(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                if let Some(&next) = chars.peek() {
                    if next == '\\' {
                        // If we encounter \', keep it as '
                        output.push('\'');
                        chars.next(); // Consume the backslash
                    } else {
                        // Convert ' to "
                        output.push('"');
                    }
                } else {
                    // Convert single ' to "
                    output.push('"');
                }
            }
            '"' => {
                // Convert " to \"
                output.push('\\');
                output.push('"');
            }
            '\\' => {
                // Handle the case for the escape character
                if let Some(&next) = chars.peek() {
                    if next == '\'' {
                        // Skip over the escape for '
                        output.push('\'');
                        chars.next(); // Consume the '
                    } else {
                        // Just add the backslash if it's not part of a valid escape sequence
                        output.push('\\');
                    }
                } else {
                    // Just add the backslash if there's nothing after it
                    output.push('\\');
                }
            }
            _ => {
                output.push(c);
            }
        }
    }

    output
}
