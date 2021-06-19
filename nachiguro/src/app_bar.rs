#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use wasm_bindgen::prelude::*;
use yew::{prelude::*, virtual_dom::VNode};

pub struct AppBar {
  props: Props,
  link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub title: String,
}

impl Component for AppBar {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
      link,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    false
  }

  fn change(&mut self, props: Self::Properties) -> bool {
    if self.props != props {
      self.props = props;
      true
    } else {
      false
    }
  }

  fn view(&self) -> Html {
    let Props {
      children,
      class,
      title,
    } = &self.props;

    html! {
      <div
        class=classes!(
          "ncgr-app-bar",
          "-active",
          class.clone(),
        )
      >
        <div class="ncgr-app-bar__title">
          { title.clone() }
        </div>
        <ul class="ncgr-app-bar__actions">
          { children.clone() }
        </ul>
      </div>
    }
  }
}