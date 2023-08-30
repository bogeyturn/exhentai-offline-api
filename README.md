# exhentai-offline-api
Uses exhentai data dumps
## Info:
This repo is an example how an offline api could look like. Its a quite advanced search and it is able to show all the info.

url: http://localhost:8080/info, body: 9, content-type: json
```json
{
    "id": 9,
    "token": "e56264c60c",
    "titles": {
        "title": "(C71) [Arisan-Antenna (Koari)] Eat The Rich! (Sukatto Golf Pangya)",
        "jpn_title": "(C71) [ありさんアンテナ (小蟻)] Eat The Rich! (スカッとゴルフ パンヤ)"
    },
    "categorize": {
        "category": "Doujinshi",
        "tags": {
            "male": ["sole male", "sunglasses"],
            "female": ["catgirl", "kemonomimi", "lolicon", "school gym uniform", "sole female", "twintails", "very long hair"],
            "other": [],
            "mixed": [],
            "temp": []
        },
        "parody": ["pangya"],
        "language": [],
        "cosplayer": [],
        "character": ["kooh"]
    },
    "owner": {
        "uploader": "Sakura",
        "groups": ["arisan-antenna"],
        "artists": ["koari"]
    },
    "pages": {
        "page_count": 14,
        "pages": [],
        "hitomi_hashs": ["\"f3191e801e1fc1342d7cddc2808171f8c0d0a94a2304beefb45b7b26d1db2512\"", "\"7d1dfc5df157c9d435bd3b9274eb386bce95905059093651dabdc77efbb500d4\"", "\"fc1bf79a8cf5e9f153fe503fe89650c9d849520b7d9f054a2b54995b2af017a2\"", "\"77b5bddcbabf23cf344b2bcf349e40f1e59b7e3effecad10a853a49f3a04a2f4\"", "\"d85dd2fd6f66198a98613efae386d753feba0f34da7a54a69014063bf2e141d8\"", "\"c8874a682b0388c3cfde01fffcbea180e358cc67a75ca8da6890dcd880d6cdea\"", "\"4e38793c15d68c8cf4dc3dfe15fdc6e452f47dbd9300fb07c9ecc79d39a8e1f5\"", "\"4c295f8440f9ae93709ca39b52bb5178a9d4fab7fab9b0435bb4463509380605\"", "\"39f58ccc657cd07d3fe3aabcd6b81da314d09ad621b6f214fd15e291eb1ee90e\"", "\"23e92dfb297ae0d3622d0a958fba321bd252910d7aad02d7f8803b028288cc5c\"", "\"3b917b8b2e239e8bedc252b1a4cedfc4ac525e0229826f3b5a373db6d0aeab17\"", "\"8bc89d8085eeeef748fd55ca245a1fea8efb3d5755b5753ac1d99e0f3ff1c621\"", "\"4a884948b88784886ea109e7b1cea589b3858ed7558931fb4d3688369ac26c79\"", "\"c208a984423ef7e26bafdfb0f7bc36dbeecf0a603b37cb0c0df287afd7001d88\""],
        "thumb": "https://ehgt.org/32/7b/327b3c0e6f33b586ef81a821d9fd0607bd464620-255758-1050-1500-jpg_l.jpg"
    },
    "rating": 4.65,
    "my_rating": null,
    "relations": {
        "variants": [1566058, 9, 548905],
        "related": [40923, 1483073, 1094832, 859567, 790360]
    },
    "deleted": false,
    "hitomi_backup": true
}
```
Search Request Structure:

```js
{
  "data": Array,
  "order": {
    "desc": bool,
    "kind": String //Options: Id, Title
  },
  "duplicate_filter": String
}
```

Array:
```js
{
  "or": bool,
  "items": Array[] | Item[]
}
```

Item:
```js
{
  "not": bool,
  "data": ItemData
}
```
ItemData:
```js
{
  "Tag": {
    "tag": String,
    "kind": String //Options: Female,Male,Mixed,Other,Rest,All
  }
}
```
ItemData:
```js
{"Artist": String}
```
All ItemData variants in rust
```rs        
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
```

<br />
<br />
Future plans:

- Frontend
- Error handling
- exhentai pages
- hitomi & exhentai fetch from web option

<br />

## Setup:

```
Download from https://huggingface.co/datasets/bogeyturn/exhentai-api-dump?not-for-all-audiences=true
failed.csv 936 kB
gallery.csv 1.45 GB
p_mixed.csv 32.8 MB

Download from https://huggingface.co/datasets/bogeyturn/Hitomila-metadata-dump?not-for-all-audiences=true
hitomi_gallery.zip

Download from https://sukebei.nyaa.si/view/3914574
gp_crawl.sqlite.7z
```

The gp_crawl is optional and need to be set in features. Put the sqlite file into crates/exhentai-api/dbs.
- Install postgres
- Create postgres database offline_doujinshi_api
- Create user postgres with no password
- Create tables in crates/exhentai-api/migrations
- Insert data into tables from csv files
