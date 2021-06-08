mod app;

use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));
  yew::start_app::<app::App>();
}
