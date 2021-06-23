use nachiguro::Image;
use yew::prelude::*;
use crate::models::Product;

pub struct ProductImageView {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub product: Product,
}

impl Component for ProductImageView {
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

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    let Props {
      product,
    } = &self.props;

    html! {
      <Image src=product.sample_image_url.to_string() />
    }
  }
}