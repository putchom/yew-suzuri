use serde::{Deserialize, Serialize};
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
use crate::api;

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

impl Product {
  pub fn get_product_list_by_user_name(user_name: &str) -> Request<Nothing> {
    let request = api::get(format!("https://suzuri.jp/api/v1/products?userName={}", user_name));

    return request;
  }
  
  pub fn get_product_info_by_id(id: i32) -> Request<Nothing> {
    let request = api::get(format!("https://suzuri.jp/api/v1/products/{}", id));
  
    return request;
  }
}
