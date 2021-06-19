use serde::{Deserialize, Serialize};
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
use crate::models::item::Item;
use crate::models::material::Material;

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
  pub id: i32,
  pub title: String,
  pub sample_image_url: String,
  pub sample_url: String,
  pub price_with_tax: i32,
  pub item: Item,
  pub material: Material
}

pub fn get_products(user_name: &str) -> Request<Nothing> {
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

pub fn get_product(id: i32) -> Request<Nothing> {
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
  let uri = format!("https://suzuri.jp/api/v1/products/{}", id);
  let request = Request::get(uri)
  .header("Authorization", authorization_value)
  .body(Nothing)
  .expect("Could not build request.");

  return request;
}
