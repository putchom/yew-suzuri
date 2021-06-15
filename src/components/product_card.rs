#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::models::product::Product;
use crate::route::Route;

use num_format::{
  Locale,
  ToFormattedString
};
use wasm_bindgen::prelude::*;
use yew::{
  prelude::*,
  html::{
    ImplicitClone
  }
};
use yew_router::prelude::*;

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

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    Self {
      props,
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
          <img class="ncgr-product-card__image" src=format!("{}", product.sample_image_url) />
        </Anchor>
        <div class="ncgr-product-card__info">
          <Anchor
            classes="ncgr-product-card__name"
            route=Route::ProductDetail(product.id)
          >
            { format!("{}", product.material.title) }
          </Anchor>
          <div class="ncgr-product-card__item-name">
            { format!("{}", product.item.humanize_name) }
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