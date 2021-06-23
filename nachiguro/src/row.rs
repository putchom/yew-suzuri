use yew::prelude::*;

pub struct Row {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
}

impl Component for Row {
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

    html! {
      <div
        class=classes!(
          "ncgr-row",
          class.clone(),
        )
      >
        { children.clone() }
      </div>
    }
  }
}