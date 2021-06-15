use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum Route {
  #[to = "/"]
  Home,
  #[to = "/search"]
  Search,
  #[to = "/cart"]
  Cart,
  #[to = "/favorite"]
  Favorite,
  #[to = "/myshop"]
  MyShop,
  #[to = "/product/{id}"]
  ProductDetail(i32),
}