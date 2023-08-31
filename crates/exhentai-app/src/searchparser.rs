use std::str::FromStr;
use crate::structs::search::{Array, Item, ItemData, ItemOrArray, TagKind};

#[test]
fn test() {
    // Search syntax:
    // category:search -> example: uploader:username uploader:"another username"
    // category!:search -> Explanation: Will exclude from search

    // categories with short
    //id,i
    //title //NOTE: if no category is given it defaults to title
    //category,c
    //group,g,
    //uploader,u
    //rating,r
    //parody,p
    //artist,a
    //"tag-all"| "tag_all"| "tagall"| "tag" | "t" | "all"
    //"tag-female" |"tag_female" |"tagfemale" | "female"| "f"
    //"tag-male" |"tag_male" |"tagmale" | "male" |"m"
    //"tag-mixed" | "tag_mixed" | "tagmixed" | "mixed"
    //"tag-other" |"tag_other" |"tagother" | "other" |"o"
    //"tag-rest" |"tag_rest" |"tagrest" | "rest"
    //language,l
    //character
    //cosplayer
    //filecount


    // space will start new item

    // use () for groups
    // or and and groups with "or:(" and "and:(" -> example: or:(uploader:username and:(uploader:"another username", tag:"some tag"))
    // groups within groups are allowed

    // parser chars that will cause problems when not handled properly: space, ", \, (, )
    // to handle these chars wrap the search in "" or handle a single char of them with \ -> Example: uploader:another\ username

    let s = ")) tag:here other:\"(sss sss)\"and:(or:())";
    println!("{:#?}", parse_items(s, true));
    panic!();
}

pub fn search_parser(s: &str, or_default:bool) -> (Array, Vec<String>) {
    let mut depth = 0;
    let mut items: Vec<ItemOrArray> = vec![];
    let mut section: Vec<char> = vec![];
    let mut double_quote = false;
    let mut errors = vec![];
    let mut push_err = |v: Result<(), String>| {
        if let Err(err) = v {
            errors.push(err);
        }
    };
    for c in s.chars() {
        if section.last() == Some(&'\\') {
            let mut count = 1;
            while count < section.len() && section[section.len() - count - 1] == '\\' {
                count += 1;
            }
            if count % 2 != 0 {
                if c == '"' {
                    section.pop();
                }
                if !double_quote && (c == '(' || c == ')') {
                    section.pop();
                }
                section.push(c);
                continue;
            }
        }
        if c == '"' {
            double_quote = !double_quote;
            if !double_quote {
                section.push(c);
                push_err(push(&mut items, UnparsedItem::Item(section.drain(..).collect()), depth, 0));
                continue;
            }
        }
        if double_quote {
            section.push(c);
            continue;
        }
        if c == ' ' {
            push_err(push(&mut items, UnparsedItem::Item(section.drain(..).collect()), depth, 0));
            continue;
        }else if c == '(' {
            depth += 1;
           if section == vec!['o', 'r', ':'] {
               push_err(push(&mut items, UnparsedItem::List(true),depth, 0));
                section.drain(..);
            }else if section == vec!['a','n','d',':'] {
               push_err(push(&mut items, UnparsedItem::List(false),depth, 0));
                section.drain(..);
            }else {
               push_err(push(&mut items, UnparsedItem::List(or_default),depth, 0));
               section.drain(..);
           }
            continue;
        }else if c  == ')' {
            push_err(push(&mut items, UnparsedItem::Item(section.drain(..).collect()),depth, 0));
            depth = depth.saturating_sub(1);
            continue;
        }
        section.push(c);
    }
    push_err(push(&mut items, UnparsedItem::Item(section.drain(..).collect()), depth, 0));
    (Array {
        or: or_default,
        items,
    }, errors)
}

#[derive(Debug)]
enum UnparsedItem {
    Item(String),
    List(bool)
}

fn push(arr: &mut Vec<ItemOrArray>, item: UnparsedItem, depth: usize, d_l: usize) -> Result<(), String> {
    if let UnparsedItem::Item(s) = &item {
        if s.is_empty() || s == " " {
            return Ok(());
        }
    }

    if let Some(ItemOrArray::Array(v)) = arr.last_mut() {
        if d_l == depth {
            arr.push(try_from_str(item)?);
        } else {
            push(&mut v.items, item, depth, d_l+1)?;
        }
    }else {
        arr.push(try_from_str(item)?);
    }

    Ok(())
}

fn try_from_str(s: UnparsedItem) -> Result<ItemOrArray, String> {
    Ok(match s {
        UnparsedItem::Item(it) => {
            let not = it.contains(":!");
            let (category, mut search) = if not {
                let (c, s) = it.split_once(":!").unwrap();
                (c, s.to_string())
            }else {
                match it.split_once(':') {
                    Some((c, s)) => (c, s.to_string()),
                    None => ("title", it)
                }
            };
            if search.starts_with('"') && search.ends_with('"') {
                search = search.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string();
            }
            let itemdata = match category.to_lowercase().as_str() {
                "id" | "i" => ItemData::Id{ related: false, id: search.parse().map_err(|_|format!("Failed to parse the id: {}", search))? },
                "title" | "" => ItemData::Title(search.to_string()),
                "category" | "c" => ItemData::Category(search.to_string()),
                "group" | "g" => ItemData::Group(search.to_string()),
                "uploader" | "u" => ItemData::Uploader(search.to_string()),
                "file-count" |  "file_count" | "filecount" => {
                    let (eq, bigger, count) = parse(&search)?;
                    ItemData::Filecount { eq, bigger, count }
                },
                "rating" | "r" => {
                    let (eq, bigger, rating) = parse(&search)?;
                    ItemData::Rating { eq, bigger, rating }
                },
                "parody" | "p" => ItemData::Parody(search.to_string()),
                "character" => ItemData::Character(search.to_string()),
                "tag-all"| "tag_all"| "tagall"| "tag" | "t" | "all" => ItemData::Tag { tag: search.to_string(), kind: TagKind::All },
                "tag-female" |"tag_female" |"tagfemale" | "female"| "f" => ItemData::Tag { tag: search.to_string(), kind: TagKind::Female },
                "tag-male" |"tag_male" |"tagmale" | "male" |"m" => ItemData::Tag { tag: search.to_string(), kind: TagKind::Male },
                "tag-mixed" | "tag_mixed" | "tagmixed" | "mixed"=> ItemData::Tag { tag: search.to_string(), kind: TagKind::Mixed },
                "tag-other" |"tag_other" |"tagother" | "other" |"o" => ItemData::Tag { tag: search.to_string(), kind: TagKind::Other },
                "tag-rest" |"tag_rest" |"tagrest" | "rest" => ItemData::Tag { tag: search.to_string(), kind: TagKind::Rest },
                "language" | "l" => ItemData::Language(search.to_string()),
                "cosplayer" => ItemData::Cosplayer(search.to_string()),
                "artist" | "a" => ItemData::Artist(search.to_string()),
                _ => return Err(format!("Category: {} not found", category))
            };
            ItemOrArray::Item(Item {
                not,
                data: itemdata,
            })
        }
        UnparsedItem::List(or) => ItemOrArray::Array(Array {
            or,
            items: vec![],
        })
    })
}

fn parse<T: FromStr>(s: &str) -> Result<(bool, bool, T), String>{
    let (str, b, s) = if let Some(v) = s.strip_prefix('>') {
        (v, true, false)
    }else if let Some(v) = s.strip_prefix('<') {
        (v, false, true)
    }else {
        (s, false, false)
    };
    let (eq, num) = if let Some(v) = str.strip_prefix('=') {
        (true, v)
    }else {
        (false, str)
    };
    Ok((eq, b == s || b, num.parse::<T>().map_err(|_|format!("Failed to parse: {}", num))?))
}