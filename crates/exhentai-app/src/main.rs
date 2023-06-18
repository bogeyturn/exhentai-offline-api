#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use crate::app::TemplateApp;

// hide console window on Windows in release
mod app;
mod exhentai_struct;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let args:Vec<String> = env::args().collect();
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    let url = args.get(1).map(|v|v.to_string());

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ExRating",
        native_options,
        Box::new(|cc| Box::new(TemplateApp::new(url, cc))),
    )
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    // send application log to console.log
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        let runner = eframe::WebRunner::new();
        let web_options = eframe::WebOptions::default();
        runner
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(TemplateApp::new(None, cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
