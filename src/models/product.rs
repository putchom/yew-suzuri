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
use crate::models::item::Item;
use crate::models::material::Material;

#[derive(Deserialize, Debug)]
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

impl Clone for Product {
  fn clone(&self) -> Self {
    Self {
      id: self.id.clone(),
      title: self.title.clone(),
      sample_image_url: self.sample_image_url.clone(),
      sample_url: self.sample_url.clone(),
      price_with_tax: self.price_with_tax.clone(),
      item: self.item.clone(),
      material: self.material.clone(),
    }
  }
}

impl Default for Product {
  fn default() -> Self {
    Self {
      id: 0,
      title: "".into(),
      sample_image_url: "".into(),
      sample_url: "".into(),
      price_with_tax: 0,
      item: Item { humanize_name: "".into() },
      material: Material { title: "".into() },
    }
  }
}

impl PartialEq for Product {
  fn eq(&self, other: &Self) -> bool {
    return self.id == other.id && self.title == other.title && self.sample_image_url == other.sample_image_url && self.sample_url == other.sample_url && self.price_with_tax == other.price_with_tax && self.item == other.item && self.material == other.material;
  }
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