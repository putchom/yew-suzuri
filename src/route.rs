use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/item/{id}"]
    ItemDetail(i32),
    #[to = "/product/{id}"]
    ProductDetail(i32),
    #[to = "/search"]
    Search,
    #[to = "/result/{query}"]
    SearchResult(String),
    #[to = "/user/{id}"]
    UserDetail(i32),
    #[to = "/"]
    Home,
}
