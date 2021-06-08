use serde::Deserialize;
use std::env;
use yew::{
  format::{
    Nothing
  },
  services::{
    fetch::{
      Request,
    }
  }
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub humanize_name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Material {
  pub title: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Product {
  pub title: String,
  pub sample_image_url: String,
  pub sample_url: String,
  pub price_with_tax: i32,
  pub item: Item,
  pub material: Material
}

impl Product {
  pub fn get_product_list(user_name: &str) -> Request<Nothing> {
    let key = "API_KEY";
    let api_key = match env::var(key) {
      Ok(val) => {
        println!("{}", "Success!");
        val
      },
      Err(e) => {
        println!("{}", e);
        "".to_string()
      },
    };
    let authorization_value = format!("{} {}", "Bearer", api_key);
    let uri = format!("https://suzuri.jp/api/v1/products?userName={}", user_name);
    let request = Request::get(uri)
    .header("Authorization", authorization_value)
    .body(Nothing)
    .expect("Could not build request.");

    return request;
  }
}