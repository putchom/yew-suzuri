#![recursion_limit = "256"]

mod api;
mod app;
mod components;
mod constants;
mod models;
mod pages;
mod route;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
