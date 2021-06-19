use anyhow::Error;
use std::env;
use yew::callback::Callback;
use yew::format::{Nothing, Json};
use yew::services::fetch::{Response, Request};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get(uri: String) -> Request<Nothing> {
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
  let uri = uri;
  let request = Request::get(uri)
  .header("Authorization", authorization_value)
  .body(Nothing)
  .expect("Could not build request.");

  return request;
}