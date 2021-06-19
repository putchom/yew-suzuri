#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Avatar {
  props: Props,
  link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub size: String,
  #[prop_or_default]
  pub src: String,
}

impl Component for Avatar {
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
      class,
      size,
      src,
    } = &self.props;

    let classes:Vec<String> = vec![
      "ncgr-avatar".to_string(),
      format!("-{}", size),
    ];

    html! {
      <span class=classes!(classes, class.clone())>
        <img
          class="ncgr-avatar__image"
          src=format!("{}", src)
        />
      </span>
    }
  }
}