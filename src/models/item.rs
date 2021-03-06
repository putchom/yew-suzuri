use crate::api;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use yew::{format::Nothing, services::fetch::Request};

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub humanize_name: String,
    pub icon_urls: HashMap<String, String>,
}

impl Item {
    pub fn get_item_list() -> Request<Nothing> {
        api::get("https://suzuri.jp/api/v1/items".to_string())
    }
}
