# exhentai-offline-api
Uses exhentai data dumps
## TODO: error handling ;D

## Info:
This repo is an example how an offline api could look like. Its a quite advanced search and it is able to show all the info.
url: http://127.0.0.1:8080/search, content-type:application/json
body:
```json
{
    "data": {
      "or": true,
      "items": [{
        "not": false,
        "data": {
          "Title": "Mirror Inma and Devil"
        }
      },
      {
        "not": false,
        "data": {
          "Tag": {
            "tag":"sole female",
            "kind": "Female"
          }
        }
      }
      ]
    },
    "order": {
      "desc": true,
      "kind": "Id"
    },
    "duplicate_filter": null
}
```

```json
[
  {
    "id": 3188148,
    "title": "Mirror Inma and Devil's Seed-part2",
    "jpn_title": "鏡の淫魔と悪魔のタネ-part2",
    "thumb": "https://ehgt.org/w/01/709/33797-ufw2bs9j.webp"
  },
  {
    "id": 3188141,
    "title": "Mirror Inma and Devil's Seed-part1",
    "jpn_title": "鏡の淫魔と悪魔のタネ-part1",
    "thumb": "https://ehgt.org/w/01/709/30469-ld96jzu8.webp"
  },
  {
    "id": 3188139,
    "title": "[NNL (Mary-san)] Asachun Mari!? | 일어났더니 마리가!? (Blue Archive) [Korean] [Team Edge] [Digital]",
    "jpn_title": "[NNL (メリサン)] 朝ちゅんマリー!? (ブルーアーカイブ) [韓国翻訳] [DL版]",
    "thumb": "https://ehgt.org/w/01/709/33102-ucdjlqh2.webp"
  },
  {
    "id": 3188138,
    "title": "[Noblood] Tiffany Valentine (Child's Play)",
    "jpn_title": null,
    "thumb": "https://ehgt.org/w/01/575/58575-324naurt.webp"
  },
  {
    "id": 3188133,
    "title": "(C105) [Navy Blue (神楽七姫)] ホタルは穹と○○したい!! (崩壊スターレイル)",
    "jpn_title": "(C105) [Navy Blue (神楽七姫)] ホタルは穹と○○したい!! (崩壊スターレイル)",
    "thumb": "https://ehgt.org/w/01/709/32707-nw022kr3.webp"
  },
  {
    "id": 3188132,
    "title": "(C105) [くわい屋 (TRNR)] 開拓クエスト・幕間 開拓者のセックス日常 続 (崩壊:スターレイル)",
    "jpn_title": "(C105) [くわい屋 (TRNR)] 開拓クエスト・幕間 開拓者のセックス日常 続 (崩壊:スターレイル)",
    "thumb": "https://ehgt.org/w/01/709/32610-lg0ddtp1.webp"
  },
  {
    "id": 3188131,
    "title": "(C105) [ろっさく工房 (六作)] 運命は開拓者の部屋で (崩壊:スターレイル)",
    "jpn_title": "(C105) [ろっさく工房 (六作)] 運命は開拓者の部屋で (崩壊:スターレイル)",
    "thumb": "https://ehgt.org/w/01/709/32609-a4suauan.webp"
  },
  {
    "id": 3188128,
    "title": "[Pot Detox] I found a girl in the club toilet after she had been raped while asleep, so I took advantage and raped her while she was asleep!!",
    "jpn_title": null,
    "thumb": "https://ehgt.org/w/01/709/32170-r1it65g2.webp"
  },
  {
    "id": 3188124,
    "title": "[Pot Detox] Shuuden Nottara Deisui Gal ga Itanode Suiminkan Rape Shichaimashita | When I got on the last train, I found a drunk girl, so I raped her while she was asleep [English] [MTL]",
    "jpn_title": "[ぽっとデトックス] 終電乗ったら泥酔ギャルがいたので睡眠姦レイプしちゃいました [英訳]",
    "thumb": "https://ehgt.org/w/01/709/31253-a56m3kwj.webp"
  },
  {
    "id": 3188121,
    "title": "[Broad smile (Sukage)] Naisho no Kyoudai Ecchi | Secret Sibling Sex [English] [Shiro Translation] [Digital]",
    "jpn_title": "[Broad smile (須影)] ないしょの姉弟えっち [英訳] [DL版]",
    "thumb": "https://ehgt.org/w/01/709/30343-6czbwysa.webp"
  }
]
```

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

## Setup:
TODO: rewrite&publish db
```
Download from https://huggingface.co/datasets/bogeyturn/Hitomila-metadata-dump?not-for-all-audiences=true
hitomi_gallery.zip

Download from https://sukebei.nyaa.si/view/3914574
gp_crawl.sqlite.7z
```

The gp_crawl is optional and need to be set in features. Put the sqlite file into crates/exhentai-api/dbs.
