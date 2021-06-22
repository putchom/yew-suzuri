use yew::prelude::*;

pub struct ListView {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub is_dense: Option<bool>,
  #[prop_or_default]
  pub is_multi_line: Option<bool>,
  #[prop_or_default]
  pub is_two_line: Option<bool>,
}

impl Component for ListView {
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
      is_dense,
      is_multi_line,
      is_two_line,
    } = &self.props;

    let classes:Vec<String> = vec![
      "ncgr-list-view".to_string(),
      match is_dense {
        Some(_) => format!("-dense"),
        None => "".to_string()
      },
      match is_multi_line {
        Some(_) => format!("-multi-line"),
        None => "".to_string()
      },
      match is_two_line {
        Some(_) => format!("-two-line"),
        None => "".to_string()
      },
    ];

    html! {
      <ul
        class=classes!(classes, class.clone())
      >
        { children.clone() }
      </ul>
    }
  }
}