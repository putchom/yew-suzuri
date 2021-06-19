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
    let uri = format!("https://suzuri.jp/api/v1/users/{}", id);
    let request = Request::get(uri)
    .header("Authorization", authorization_value)
    .body(Nothing)
    .expect("Could not build request.");
  
    return request;
  }
}