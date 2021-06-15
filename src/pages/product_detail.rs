use nachiguro::{Container, Heading};
use yew::prelude::*;

pub struct ProductDetail {
  props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub id: i32,
}

impl Component for ProductDetail {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self {
      props,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> bool {
    unimplemented!()
  }

  fn change(&mut self, _props: Self::Properties) -> bool {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <Container>
          <Heading level=1 size={"m"}>{ &self.props.id }</Heading>
        </Container>
      </div>
    }
  }
}