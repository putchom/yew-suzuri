use anyhow::Error;
use yew::callback::Callback;
use yew::format::Json;
use yew::services::fetch::Response;

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;
