use crate::exhentai_struct::ExHentaiResponse;
use egui::scroll_area::ScrollBarVisibility;
use egui::{Key, ScrollArea, Spinner, Ui};
use egui_extras::{Column, RetainedImage, TableBuilder};
use poll_promise::Promise;
use serde::Serialize;
use std::collections::{HashSet, VecDeque};
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
type Images = Arc<Mutex<Vec<Arc<Mutex<Option<RetainedImage>>>>>>;
type ImageLoader = Arc<Mutex<Option<Arc<AtomicBool>>>>;

pub struct Data {
    promise: SubPromise,
    image_loader: ImageLoader,
    images: Images,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            promise: SubPromise { promise: None },
            image_loader: Arc::new(Mutex::new(None)),
            images: Arc::new(Mutex::new(vec![])),
        }
    }
}
pub struct TemplateApp {
    data: VecDeque<Data>,
    rating: Option<Rating>,
    rating_req: Option<Promise<()>>,
    host: String,
    scroll_to: bool,
    columns: usize,
    keys: HashSet<Key>,
}

struct SubPromise {
    promise: Option<Promise<Result<ExHentaiResponse, reqwest::Error>>>,
}

#[derive(Serialize)]
struct EntryRequestOption {
    entry: Option<i32>,
}

impl SubPromise {
    fn get_promise(
        &mut self,
        ctx: &egui::Context,
        images: Images,
        id: Option<i32>,
        host: String,
    ) -> &Promise<Result<ExHentaiResponse, reqwest::Error>> {
        if self.promise.is_none() {
            self.promise = Some(self.promise_data(
                ctx,
                images,
                format!("{}get_next_entry", host),
                EntryRequestOption { entry: id },
            ));
        }
        self.promise.as_ref().unwrap()
    }

    fn promise_data(
        &mut self,
        ctx: &egui::Context,
        images: Images,
        url: String,
        json: EntryRequestOption,
    ) -> Promise<Result<ExHentaiResponse, reqwest::Error>> {
        let (s, r) = Promise::new();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let v: Result<ExHentaiResponse, reqwest::Error> =
                client.post(url).json(&json).send().unwrap().json();
            let mut imgs = vec![];
            if let Ok(v) = &v {
                for _ in 0..v.pages.page_count as usize {
                    imgs.push(Arc::new(Mutex::new(None)))
                }
            }
            while images.lock().unwrap().pop().is_some() {}
            images.lock().unwrap().append(&mut imgs);
            s.send(v);
            ctx.request_repaint();
        });
        r
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            data: VecDeque::from([
                Data::default(),
                Data::default(),
                Data::default(),
                Data::default(),
                Data::default(),
            ]),
            rating: None,
            rating_req: None,
            host: "http://127.0.0.1:8080/".to_string(),
            scroll_to: true,
            columns: 2,
            keys: HashSet::new(),
        }
    }
}

impl TemplateApp {
    pub fn new(host: Option<String>, _: &eframe::CreationContext<'_>) -> Self {
        let mut default = Self::default();
        if let Some(host) = host {
            default.host = host;
        }
        default
    }

    pub fn shift(&mut self) {
        if let Some(v) = self.data.pop_front() {
            self.rating = None;
            // stops image loading thread
            if let Some(v) = &*v.image_loader.lock().unwrap() {
                v.store(false, Ordering::Relaxed);
            }
            self.data.push_back(Data::default());
            self.scroll_to = true;
        }
    }

    fn add_column(&mut self) {
        self.columns += 1;
    }

    fn remove_column(&mut self) {
        if self.columns > 1 {
            self.columns -= 1;
        }
    }
    pub fn get_first_and_load(&mut self, ctx: &egui::Context) -> &mut Data {
        let mut id: Option<i32> = None;
        let mut item = None;
        let mut load_next = true;
        for item_iter in &mut self.data {
            let end = {
                let item = item_iter.promise.get_promise(
                    ctx,
                    item_iter.images.clone(),
                    id,
                    self.host.clone(),
                );
                if let Some(Ok(v)) = item.ready() {
                    if item_iter.image_loader.lock().unwrap().is_none() && load_next {
                        let atomic = Arc::new(AtomicBool::new(true));
                        downloader(
                            item_iter.images.lock().unwrap().clone(),
                            atomic.clone(),
                            HitomiImages {
                                hashs: v
                                    .pages
                                    .pages
                                    .iter()
                                    .map(|page| page.1.to_string())
                                    .collect(),
                            },
                            self.host.clone(),
                        );
                        item_iter.image_loader.lock().unwrap().replace(atomic);
                        load_next = false;
                    }
                    if let Some(v) = &*item_iter.image_loader.lock().unwrap() {
                        load_next = !v.load(Ordering::Relaxed);
                    }
                    id = Some(v.id);
                    false
                } else {
                    true
                }
            };
            if item.is_none() {
                item = Some(item_iter);
            }
            if end {
                break;
            }
        }
        item.unwrap()
    }

    pub fn send(&mut self, ctx: &egui::Context, rating: Rating, host: String) {
        let (s, r) = Promise::new();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let _ = client
                .post(format!("{}add_rating", host))
                .json(&rating)
                .send();
            s.send(());
            ctx.request_repaint();
        });
        self.rating_req = Some(r)
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Id {
    id: Option<i32>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Rating {
    pub id: i32,
    same: Option<String>,
    other_lang: Option<String>,
    related: Option<String>,
    rating: i32,
}

impl Rating {
    fn new_rating(&self, rating: i32) -> Self {
        Self {
            id: self.id,
            same: self.same.clone(),
            other_lang: self.other_lang.clone(),
            related: self.related.clone(),
            rating,
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let keys = ctx.input(|input| {
                let v = &input.keys_down;
                let items = v
                    .iter()
                    .filter_map(|v| match self.keys.contains(v) {
                        true => None,
                        false => Some(*v),
                    })
                    .collect::<HashSet<_>>();
                self.keys = v.clone();
                items
            });
            if keys.contains(&Key::J) {
                self.remove_column();
            } else if keys.contains(&Key::K) {
                self.add_column();
            }
            if let Some(v) = &self.rating_req {
                if v.ready().is_some() {
                    self.rating_req = None;
                    self.shift();
                }
            } else if let Some(rating) = &self.rating {
                let rating = {
                    if keys.contains(&Key::Space) {
                        Some(rating.new_rating(0))
                    } else if keys.contains(&Key::Num1) {
                        Some(rating.new_rating(1))
                    } else if keys.contains(&Key::Num2) {
                        Some(rating.new_rating(2))
                    } else if keys.contains(&Key::Num3) {
                        Some(rating.new_rating(3))
                    } else if keys.contains(&Key::Num4) {
                        Some(rating.new_rating(4))
                    } else if keys.contains(&Key::Num5) {
                        Some(rating.new_rating(5))
                    } else {
                        None
                    }
                };
                if let Some(rating) = rating {
                    self.send(ctx, rating, self.host.clone());
                }
            }
            let show = self.get_first_and_load(ctx);
            let promise = show.promise.promise.as_ref().unwrap();
            let mut rating = None;
            if let Some(ready) = promise.ready() {
                match ready {
                    Ok(v) => {
                        let cto = |ve: Vec<i32>| match ve.is_empty() {
                            true => None,
                            false => Some(
                                serde_json::to_string(
                                    &ve.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
                                )
                                .unwrap(),
                            ),
                        };
                        rating = Some(Rating {
                            id: v.id,
                            same: cto(v.relations.variants.clone()),
                            other_lang: cto(v.relations.languages.clone()),
                            related: cto(v.relations.related.clone()),
                            rating: 0,
                        });
                        let mut items = { show.images.lock().unwrap().clone() };
                        ui.vertical(|ui| {
                            ui.heading(format!(
                                "Title: {}",
                                v.titles.title.as_ref().unwrap_or(&"Unknown".to_string())
                            ));
                            ui.label(format!(
                                "Category: {}",
                                v.categorize
                                    .category
                                    .as_ref()
                                    .unwrap_or(&"Unknown".to_string())
                            ));
                            scrollable("Tags Male", &v.categorize.tags.male, ui);
                            scrollable("Tags Female", &v.categorize.tags.female, ui);
                            scrollable("Tags Other", &v.categorize.tags.other, ui);
                            scrollable("Tags Mixed", &v.categorize.tags.mixed, ui);
                            scrollable("Tags Temp", &v.categorize.tags.temp, ui);
                            scrollable("Parodies", &v.categorize.parody, ui);
                            ui.heading(format!("Rating: {}", v.rating.unwrap_or(0.0)));
                        });
                        let screen = frame.info().window_info.size;
                        let colums_height = (screen.x / self.columns as f32) * 1.5;
                        let mut builder = TableBuilder::new(ui);
                        for _ in 0..self.columns {
                            builder = builder.column(Column::remainder());
                        }
                        if self.scroll_to {
                            builder = builder.vertical_scroll_offset(0.);
                            self.scroll_to = false;
                        }
                        builder.body(|mut body| {
                            while !items.is_empty() {
                                body.row(colums_height, |mut row| {
                                    for _ in 0..self.columns as usize {
                                        if items.is_empty() {
                                            break;
                                        }
                                        let image = items.remove(0);
                                        row.col(|ui| {
                                            let available_size = ui.available_size();
                                            match &*image.lock().unwrap() {
                                                None => {
                                                    let spinner = Spinner::new();
                                                    ui.add(spinner);
                                                }
                                                Some(img) => {
                                                    img.show_size(ui, available_size);
                                                }
                                            }
                                        });
                                    }
                                });
                            }
                        });
                    }
                    Err(err) => {
                        ui.heading("An error occurred");
                        ui.label(err.to_string());
                    }
                }
            } else {
                let spinner = Spinner::new();
                ui.add(spinner);
            }
            if self.rating.is_none() {
                self.rating = rating;
            }
        });
    }
}

fn scrollable(id: &str, items: &Vec<String>, ui: &mut Ui) {
    if items.is_empty() {
        return;
    }
    ui.horizontal(|ui| {
        ui.label(format!("{}: ", id));
        ScrollArea::horizontal()
            .auto_shrink([false; 2])
            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
            .id_source(id)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for item in items {
                        if ui.button(item).clicked() {
                            //TODO:
                        }
                    }
                })
            });
    });
}

fn downloader(
    mut items: Vec<Arc<Mutex<Option<RetainedImage>>>>,
    running: Arc<AtomicBool>,
    hashs: HitomiImages,
    host: String,
) {
    std::thread::spawn(move || {
        let chunksize = 2048;
        let mut data = vec![0u8; chunksize];
        let mut req = reqwest::blocking::Client::new()
            .post(format!("{}get_hitomi_images", host))
            .json(&hashs)
            .send()
            .unwrap();
        let mut len = None;
        let mut buffer = vec![];
        let mut count = 0;
        loop {
            if !running.load(Ordering::Relaxed) {
                break;
            }
            let read = req.read(&mut data);
            let l = read.unwrap_or(0);
            if l == 0 {
                break;
            }
            buffer.append(&mut data[..l].to_vec());
            if let Some(v) = len {
                if v <= buffer.len() as u64 {
                    let drained = buffer.drain(..v as usize).collect::<Vec<_>>();
                    generate_image(drained, hashs.hashs[count].clone(), items.remove(0));
                    count += 1;
                    len = None;
                }
            }
            if len.is_none() && buffer.len() >= 8 {
                let drained = buffer.drain(..8);
                len = Some(u64::from_be_bytes(drained.as_slice().try_into().unwrap()))
            }
        }
        running.store(false, Ordering::Relaxed)
    });
}
fn generate_image(bytes: Vec<u8>, name: String, item: Arc<Mutex<Option<RetainedImage>>>) {
    std::thread::spawn(move || {
        let image = image::load_from_memory(&bytes).map_err(|err| err.to_string());
        if let Ok(image) = image {
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let ci = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            let img = RetainedImage::from_color_image(name, ci);
            item.lock().unwrap().replace(img);
        } else {
            println!("{}", name);
        }
    });
}

#[derive(Serialize)]
struct HitomiImages {
    hashs: Vec<String>,
}
