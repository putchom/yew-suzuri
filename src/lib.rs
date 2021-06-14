mod app;
mod components;
mod models;
mod pages;

use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  yew::start_app::<app::App>();
}
