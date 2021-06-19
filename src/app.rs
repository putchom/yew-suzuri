use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::Header;
use crate::pages::{Home, ProductDetailPage};
use crate::route::Route;

pub struct App {}

impl Component for App {
  type Message = ();
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {}
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {

    let render = Router::render(move |switch: Route| match switch {
      Route::ProductDetailPage(id) => {
        html! {<ProductDetailPage id=id />}
      }
      Route::HomePage => {
        html! {<Home />}
      }
    });

    html! {
      <>
        <Header />
        <Router<Route, ()> render=render />
      </>
    }
  }
}
