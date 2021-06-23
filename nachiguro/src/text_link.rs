use yew::prelude::*;

pub struct TextLink {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
}

impl Component for TextLink {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      props,
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
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
      "ncgr-text-link".to_string(),
    ];

    html! {
      <a
        class=classes!(classes, class.clone())
        href="#"
      >
        { children.clone() }
      </a>
    }
  }
}