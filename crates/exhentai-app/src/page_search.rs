use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use eframe::Frame;
use egui::{Context, Sense, Spinner, TextEdit, Ui, vec2};
use egui_extras::{Column, RetainedImage, TableBuilder};
use poll_promise::Promise;
use url::Url;
use crate::searchparser::search_parser;
use crate::structs::search::{Array, Item, ItemData, ItemOrArray, SearchRequest, SearchResponse, TagKind};

const MAX_CONN: usize = 4;
pub struct ExHentaiRequester {
    instances: usize,
    connections: [Option<(i32, Promise<Option<RetainedImage>>)>; MAX_CONN],
    cookie: String
}

impl Default for ExHentaiRequester {
    fn default() -> Self {
        Self {
            instances: 0,
            connections: [None, None, None, None],
            cookie: "".to_string(),
        }
    }
}

impl ExHentaiRequester {
    fn new_conn(&mut self, id: i32, url: &str) {
        if self.instances >= MAX_CONN {
            return;
        }
        let mut request = ehttp::Request::get(url);
        request.headers.insert("Cookie".into(), self.cookie.as_str().into());
        let (sender, receiver) = Promise::new();
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            sender.send(from_response(result.unwrap()));
        });
        self.instances += 1;
        let mut i = 0;
        while self.connections[i].is_some() {
            i += 1;
        }
        if i >= MAX_CONN {
            return;
        }
        self.connections[i] = Some((id, receiver));
    }
    fn take_if_ready(&mut self, hm: &mut HashMap<i32, RetainedImage>) {
        for i in 0..self.connections.len() {
            if let Some((_, v)) = &self.connections[i] {
                if v.ready().is_some() {
                    let v = self.connections.get_mut(i).unwrap();
                    self.instances -= 1;
                    let v = mem::take(v).unwrap();
                    hm.insert(v.0, v.1.block_and_take().unwrap());
                }
            }
        }
    }
}

pub fn search_page(ui: &mut Ui, sp: &mut SearchPage, api_url: &Arc<Url>, columns: usize, frame: &mut Frame) {
    if ui.add(TextEdit::singleline(&mut sp.text_edit).margin(vec2(10.,10.)).desired_width(ui.available_width())).lost_focus() {
        let (data, err) = search_parser(&sp.text_edit, sp.or_default);
        let mut arr = Array {
            or: false,
            items: vec![ItemOrArray::Array(data)],
        };
        if sp.full_color {
            arr.items.push(ItemOrArray::Item(Item { not: false, data: ItemData::Tag { tag: "full color".to_string(), kind: TagKind::Other } }));
        }
        sp.reset = true;
        sp.searched = false;
        sp.errors = err;
        sp.search_query.data = arr;
    }

    if ui.checkbox(&mut sp.full_color, "full color").changed() {
        sp.reset = true;
        sp.searched = false;
        let item = Item { not: false, data: ItemData::Tag { tag: "full color".to_string(), kind: TagKind::Other } };
        let mut pos = None;
        for (i, it) in sp.search_query.data.items.iter().enumerate() {
            if let ItemOrArray::Item(data) = it {
                if data == &item {
                    pos = Some(i);
                    break;
                }
            }
        }
        match sp.full_color {
            true => {
                if pos.is_none() {
                    sp.search_query.data.items.push(ItemOrArray::Item(item));
                }
            }
            false => {
                if let Some(v) = pos {
                    sp.search_query.data.items.remove(v);
                }
            }
        }
    }
    if ui.checkbox(&mut sp.or_default, "Or group default").changed() {
        sp.reset = true;
        sp.searched = false;
        let (data, _) = search_parser(&sp.text_edit, sp.or_default);
        let mut arr = Array {
            or: false,
            items: vec![ItemOrArray::Array(data)],
        };
        if sp.full_color {
            arr.items.push(ItemOrArray::Item(Item { not: false, data: ItemData::Tag { tag: "full color".to_string(), kind: TagKind::Other } }));
        }
        sp.search_query.data = arr;
    }

    sp.get_from_api(api_url.clone(), ui.ctx());

    sp.ex.take_if_ready(&mut sp.images);

    let mut builder = TableBuilder::new(ui);
    for _ in 0..columns {
        builder = builder.column(Column::remainder());
    }
    if sp.reset_scroll {
        builder = builder.vertical_scroll_offset(0.);
        sp.reset_scroll = false;
    }
    let screen = frame.info().window_info.size;
    let colums_height = (screen.x / columns as f32) * 1.5;
    builder.body(|mut body| {
        for row_ in sp.search_responses.chunks(columns) {
            body.row(colums_height, |mut row| {
                for item in row_ {
                    row.col(|ui| {
                        let available_size = ui.available_size();
                        if let Some(v) = sp.images.get(&item.id){
                            v.show_size(ui, available_size);
                        }else {
                            if let Some(v) = &item.thumb {
                                sp.ex.new_conn(item.id, v);
                            }
                            let (area, _) = ui.allocate_exact_size(available_size, Sense::click());
                            let spinner = Spinner::new();
                            ui.put(area, spinner);
                        }
                        ui.label(&item.title);
                    });
                }
            });
        }
    });
}

pub struct SearchPage {
    pub(crate) search_query: SearchRequest,
    pub(crate) promise: Option<Promise<Vec<SearchResponse>>>,
    pub ex: ExHentaiRequester,
    pub images: HashMap<i32, RetainedImage>,
    pub(crate) search_responses: Vec<SearchResponse>,
    pub(crate) searched: bool,
    pub(crate) text_edit: String,
    pub full_color: bool,
    pub or_default: bool,
    pub errors: Vec<String>,
    pub reset_scroll: bool,
    pub reset: bool,
}

impl SearchPage {
    fn get_from_api(&mut self, url: Arc<Url>, ctx: &Context) {
        if let Some(v) = &self.promise {
            if v.ready().is_some() {
                self.searched = true;
                let mut res = self.promise.take().unwrap().block_and_take();
                if self.reset {
                    self.reset = false;
                    self.reset_scroll = true;
                    self.search_responses = res;
                }else {
                    self.search_responses.append(&mut res);
                }
            }
        }
        if self.promise.is_some() {
            return;
        }
        if self.searched {
            return;
        }
        let (sender, receiver) = Promise::new();
        let mut request = ehttp::Request::post(url.join("search").unwrap().to_string(), serde_json::to_string(&self.search_query).unwrap().into_bytes());
        request.headers.insert("Content-Type".into(), "application/json".into());
        let ctx = ctx.clone();
        self.promise = Some(receiver);
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            sender.send(serde_json::from_slice(&result.unwrap().bytes).unwrap());
            ctx.request_repaint();
        });
    }
}

fn from_response(response: ehttp::Response) -> Option<RetainedImage> {
    let content_type = response.content_type().unwrap_or_default();
    if content_type.starts_with("image/") {
        RetainedImage::from_image_bytes(&response.url, &response.bytes).ok()
    } else {
        None
    }
}