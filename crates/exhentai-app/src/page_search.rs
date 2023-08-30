use std::sync::Arc;
use egui::{Context, Ui};
use poll_promise::Promise;
use url::Url;
use crate::structs::search::{SearchRequest, SearchResponse};

pub fn search_page(ui: &mut Ui, sp: &mut SearchPage, api_url: &Arc<Url>) {
    sp.get_from_api(api_url.clone(), ui.ctx());
    for item in &sp.search_responses {
        ui.label(&item.title);
    }
}

pub struct SearchPage {
    pub(crate) search_query: SearchRequest,
    pub(crate) promise: Option<Promise<Vec<SearchResponse>>>,
    pub(crate) search_responses: Vec<SearchResponse>,
    pub(crate) searched: bool
}

impl SearchPage {
    fn get_from_api(&mut self, url: Arc<Url>, ctx: &Context) {
        if let Some(v) = &self.promise {
            if v.ready().is_some() {
                self.searched = true;
                let mut res = self.promise.take().unwrap().block_and_take();
                self.search_responses.append(&mut res);
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
        ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
            sender.send(serde_json::from_slice(&result.unwrap().bytes).unwrap());
            ctx.request_repaint();
        });
        self.promise = Some(receiver);
    }
}