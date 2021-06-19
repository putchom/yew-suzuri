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

use crate::api;

#[derive(Deserialize, Serialize, Clone, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
  pub id: i32,
  pub name: String,
  pub display_name: Option<String>,
  pub avatar_url: Option<String>,
}

impl User {
  pub fn get_user_info_by_id(id: i32) -> Request<Nothing> {
    let request = api::get(format!("https://suzuri.jp/api/v1/users/{}", id));
  
    return request;
  }
}