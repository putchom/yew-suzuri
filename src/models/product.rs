use crate::api;
use crate::models::{item::Item, material::Material};
use serde::{Deserialize, Serialize};
use yew::{format::Nothing, services::fetch::Request};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub sample_image_url: String,
    pub sample_url: String,
    pub price_with_tax: i32,
    pub item: Item,
    pub material: Material,
}

impl Product {
    pub fn get_product_list_by_item_id(item_id: i32) -> Request<Nothing> {
        api::get(format!(
            "https://suzuri.jp/api/v1/products?itemId={}",
            item_id
        ))
    }

    pub fn get_product_list_by_user_name(user_name: &str) -> Request<Nothing> {
        api::get(format!(
            "https://suzuri.jp/api/v1/products?userName={}",
            user_name
        ))
    }

    pub fn get_product_list_by_query(query: &str) -> Request<Nothing> {
        api::get(format!(
            "https://suzuri.jp/api/v1/products/search?q={}",
            query
        ))
    }

    pub fn get_product_info_by_id(id: i32) -> Request<Nothing> {
        api::get(format!("https://suzuri.jp/api/v1/products/{}", id))
    }
}
