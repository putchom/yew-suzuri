use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
  #[to = "/product/{id}"]
  ProductDetail(i32),
  #[to = "/user/{id}"]
  UserDetail(i32),
  #[to = "/"]
  HomePage,
}
