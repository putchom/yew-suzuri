use nachiguro::{
  Card,
  Container,
  Heading,
  Paragraph,
  TextLink
};
use crate::models::Product;
use num_format::{
  Locale,
  ToFormattedString
};

use yew::prelude::*;

pub struct ProductInfoCard {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub product: Product,
}

impl Component for ProductInfoCard {
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
      <Card class=classes!("ProductDetail-product-card") color="secondary-grouped-background".to_string()>
        <Container>
          <Heading level=1 size="m">
            { format!("{}", product.material.title) }
          </Heading>
          <Paragraph>
            <TextLink>
              { format!("{}", product.item.humanize_name) }
            </TextLink>
          </Paragraph>
          <Paragraph>
            { format!("{}{}", product.price_with_tax.to_formatted_string(&Locale::en), "円(税込)") }
          </Paragraph>
          {
            match &product.material.description {
              Some(description) => html! {
                <div class=classes!("ProductDetail-description")>
                  <Heading level=2 size="s">
                    { "このアイテムについて" }
                  </Heading>
                  <Paragraph>
                    { format!("{}", description) }
                  </Paragraph>
                </div>
              },
              None => html! {
                <div></div>
              },
            }
          }
        </Container>
      </Card>
    }
  }
}