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
  product_detail::ProductDetail,
  search::Search,
};
use crate::route::Route;

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
      Route::ProductDetail(id) => html! { <ProductDetail id=id /> }
    });

    type Anchor = RouterAnchor<Route>;

    html! {
      <>
        <AppBar title="SUZURI">
          <li>
            <Anchor route=Route::Home>
              { "ホーム" }
            </Anchor>
          </li>
          <li>
            <Anchor route=Route::Search>
              { "さがす" }
            </Anchor>
          </li>
          <li>
            <Anchor route=Route::Cart>
              { "カート" }
            </Anchor>
          </li>
          <li>
            <Anchor route=Route::Favorite>
              { "ズッキュン" }
            </Anchor>
          </li>
          <li>
            <Anchor route=Route::MyShop>
              { "マイショップ" }
            </Anchor>
          </li>
        </AppBar>
        <Router<Route, ()> render=render />
      </>
    }
  }
}