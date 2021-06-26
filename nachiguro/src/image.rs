use yew::prelude::*;

pub struct Image {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub height: Option<String>,
  #[prop_or_default]
  pub src: String,
  #[prop_or_default]
  pub width: Option<String>,
}

impl Component for Image {
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
      class,
      height,
      src,
      width,
    } = &self.props;

    let classes:Vec<String> = vec![
      "ncgr-image".to_string(),
    ];

    let styles:Vec<String> = vec![
      match height {
        Some(height) => format!("height: {};", height),
        None => "".to_string()
      },
      match width {
        Some(width) => format!("width: {};", width),
        None => "".to_string()
      },
    ];

    html! {
      <img
        class=classes!(classes, class.clone())
        src=format!("{}", src)
        style=styles.join(" ")
      />
    }
  }
}