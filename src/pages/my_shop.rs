use nachiguro::{Container, Heading};
use yew::prelude::*;

pub struct MyShop;

impl Component for MyShop {
  type Message = ();
  type Properties = ();

  fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
    Self
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
          <Heading level=1 size={"m"}>{ "マイショップ" }</Heading>
        </Container>
      </div>
    }
  }
}