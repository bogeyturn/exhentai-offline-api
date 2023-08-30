use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResponse {
    pub id: i32,
    pub title: String,
    pub jpn_title: Option<String>,
    pub thumb: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub(crate) data: Array,
    pub(crate) order: Order,
    pub(crate) duplicate_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FilterRequest {
    filter: Vec<String>,
    name: String,
}


#[derive(Serialize, Deserialize)]
pub struct Order {
    pub(crate) desc: bool,
    pub(crate) kind: OrderKind,
}

#[derive(Serialize, Deserialize)]
pub enum OrderKind {
    Id,
    Title,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemOrArray {
    Item(Item),
    Array(Array),
}

#[derive(Serialize, Deserialize)]
pub struct Array {
    pub(crate) or: bool,
    pub(crate) items: Vec<ItemOrArray>,
}


#[derive(Serialize, Deserialize)]
pub struct Item {
    not: bool,
    data: ItemData,
}

#[derive(Serialize, Deserialize)]
pub enum ItemData {
    Id { related: bool, id: i32 },
    Title(String),
    Category(String),
    Artist(String),
    Group(String),
    Uploader(String),
    Filecount { eq: bool, bigger: bool, count: i32 },
    Rating { eq: bool, bigger: bool, rating: f32 },
    Parody(String),
    Character(String),
    Tag { tag: String, kind: TagKind },
    Language(String),
    Cosplayer(String),
}

#[derive(Serialize, Deserialize)]
pub enum TagKind {
    Female,
    Male,
    Mixed,
    Other,
    Rest,
    All
}