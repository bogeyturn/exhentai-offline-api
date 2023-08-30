use std::collections::HashSet;
use std::sync::Arc;
use eframe::{App, Frame};
use egui::{Context, Key, Response, Ui, Widget};
use poll_promise::Promise;
use crate::page_info::info_page;
use crate::page_search::{search_page, SearchPage};
use crate::structs::search::{Array, Order, OrderKind, SearchRequest, SearchResponse};
use url::Url;

pub struct GlobalStorage {
    search: SearchPage,
    open_page_gid: Option<i32>,
    api_url: Arc<Url>,
    columns: usize,
    keys: HashSet<Key>,
}

impl App for GlobalStorage {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
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

        egui::CentralPanel::default().show(ctx, |ui|  {
            if self.open_page_gid.is_some() {
                info_page(ui);
            }else {
                search_page(ui, &mut self.search, &self.api_url);
            }
        });
    }
}

impl Default for GlobalStorage {
    fn default() -> Self {
        Self {
            search: SearchPage {
                search_query: SearchRequest {
                    data: Array {
                        or: false,
                        items: vec![],
                    },
                    order: Order {
                        desc: true,
                        kind: OrderKind::Id,
                    },
                    duplicate_filter: None,
                },
                promise: None,
                search_responses: vec![],
                searched: false,
            },
            open_page_gid: None,
            api_url: Url::parse("http://127.0.0.1:8080").unwrap().into(),
            columns: 2,
            keys: Default::default(),
        }
    }
}

impl GlobalStorage {
    pub(crate) fn new(api_url: Option<Url>) -> Self {
        if let Some(api_url) = api_url {
            GlobalStorage {
                api_url: api_url.into(),
                ..GlobalStorage::default()
            }
        }else {
            Default::default()
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
}