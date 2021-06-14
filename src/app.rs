#![allow(dead_code)]

use nachiguro::AppBar;
use yew::{
  prelude::*,
  html::ComponentLink,
  virtual_dom::vnode::VNode,
};
use yew_router::prelude::*;
use crate::pages::{
  cart::Cart,
  favorite::Favorite,
  home::Home,
  my_shop::MyShop,
  search::Search,
};

pub struct App;

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _props: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> VNode {
    let render = Router::render(|switch: Route| match switch {
      Route::Home => html! { <Home /> },
      Route::Search => html! { <Search /> },
      Route::Cart => html! { <Cart /> },
      Route::Favorite => html!{ <Favorite /> },
      Route::MyShop => html! { <MyShop /> },
    });

    html! {
      <>
        <AppBar title="SUZURI">
          <li>
            <RouterAnchor<Route> route=Route::Home>
              { "ホーム" }
            </RouterAnchor<Route>>
          </li>
          <li>
            <RouterAnchor<Route> route=Route::Search>
              { "さがす" }
            </RouterAnchor<Route>>
          </li>
          <li>
            <RouterAnchor<Route> route=Route::Cart>
              { "カート" }
            </RouterAnchor<Route>>
          </li>
          <li>
            <RouterAnchor<Route> route=Route::Favorite>
              { "ズッキュン" }
            </RouterAnchor<Route>>
          </li>
          <li>
            <RouterAnchor<Route> route=Route::MyShop>
              { "マイショップ" }
            </RouterAnchor<Route>>
          </li>
        </AppBar>
        <Router<Route, ()> render=render />
      </>
    }
  }
}

#[derive(Switch, Clone)]
enum Route {
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
}