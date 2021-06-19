#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Paragraph {
  props: Props,
  link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
}

impl Component for Paragraph {
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
    } = &self.props;

    let classes:Vec<String> = vec![
      "ncgr-typography".to_string(),
    ];

    html! {
      <p
        class=classes!(classes, class.clone())
      >
        { children.clone() }
      </p>
    }
  }
}