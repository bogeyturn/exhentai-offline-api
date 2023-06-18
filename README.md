# exhentai-offline-api
Uses exhentai data dumps
Info:
This repo is an example how a offlien api could look like. Many things that could be implemented arent. For example a search feature that is better than the exhentai search could be easily added. I only build this project yesterday. Feel free to extend this project. As of now it only the options to preview and rate the images. Its just a proof of concept and by no means safe code. 
<br />
Setup:
Download from https://archive.org/details/panda-metadata-thumbnails-dump-2023-v2
api_dump.sqlite	14-Jan-2023 13:12	1.3G
failed.sqlite	14-Jan-2023 13:13	1.1M
gp_crawl.sqlite.7z	14-Jan-2023 13:34	8.3G(for image links/alternative fetches from hitomi.la)

The gp_crawl is optional(but usage needs to be set in crates/exhentai-api/src/main.rs) when establishing connections. Put the databases into crates/exhentai-api/dbs.
