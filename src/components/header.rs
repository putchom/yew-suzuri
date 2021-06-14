#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use nachiguro::{AppBar};
use num_format::{Locale, ToFormattedString};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::html::{ImplicitClone};

pub struct Header {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub class: Classes,
}

impl Component for Header {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
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
      class,
    } = &self.props;

    html! {
      <AppBar>{ "SUZURI" }</AppBar>
    }
  }
}