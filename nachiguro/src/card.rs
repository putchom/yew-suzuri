use yew::prelude::*;

pub struct Card {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub color: Option<String>,
}

impl Component for Card {
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
      color,
    } = &self.props;

    let classes:Vec<String> = vec![
      "ncgr-card".to_string(),
      match color {
        Some(color) => format!("ncgr-card--{}", color),
        None => "".to_string()
      },
    ];

    html! {
      <div
        class=classes!(classes, class.clone())
      >
        { children.clone() }
      </div>
    }
  }
}