use num_format::{
  Locale,
  ToFormattedString
};
use yew::{
  prelude::*,
  html::ImplicitClone,
};
use yew_router::prelude::*;
use crate::models::product::Product;
use crate::route::Route;

pub struct ProductCard {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub product: Product,
}

impl Component for ProductCard {
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
      class,
      product,
    } = &self.props;

    type Anchor = RouterAnchor<Route>;

    html! {
      <div
        class=classes!(
          "ncgr-product-card",
          class.clone(),
        )
      >
        <Anchor
          classes="ncgr-product-card__thumbnail"
          route=Route::ProductDetail(product.id)
        >
          <img class="ncgr-product-card__image" src=product.sample_image_url.to_string() />
        </Anchor>
        <div class="ncgr-product-card__info">
          <Anchor
            classes="ncgr-product-card__name"
            route=Route::ProductDetail(product.id)
          >
            { product.material.title.to_string() }
          </Anchor>
          <div class="ncgr-product-card__item-name">
            { product.item.humanize_name.to_string() }
          </div>
          <div class="ncgr-product-card__price">
            { format!("{}{}", product.price_with_tax.to_formatted_string(&Locale::en), "円") }
          </div>
        </div>
      </div>
    }
  }
}

impl ImplicitClone for Product {}