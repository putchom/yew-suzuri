#![allow(dead_code)]

use crate::constants::API_KEY;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get(uri: String) -> Request<Nothing> {
    Request::get(uri)
        .header("Authorization", format!("{} {}", "Bearer", API_KEY))
        .body(Nothing)
        .expect("Could not build request.")
}
