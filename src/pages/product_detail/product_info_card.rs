use crate::models::product::Product;
use nachiguro::{
    types::{background_color::BackgroundColor, heading_level::HeadingLevel, size::Size},
    Button, Card, Container, Heading, Paragraph, TextLink,
};
use num_format::{Locale, ToFormattedString};
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
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let Props { product } = &self.props;

        html! {
            <Card
                background_color=BackgroundColor::SecondaryGroupedBackground
                class=classes!("ProductDetail-product-card")
            >
                <Container>
                    <Heading
                        level=HeadingLevel::One
                        size=Size::M
                    >
                        { product.material.title.to_string() }
                    </Heading>
                    <Paragraph>
                        <TextLink>
                        { product.item.humanize_name.to_string() }
                        </TextLink>
                    </Paragraph>
                    <Paragraph>
                        { format!("{}{}", product.price_with_tax.to_formatted_string(&Locale::en), "円(税込)") }
                    </Paragraph>
                    <div class=classes!("ProductDetail-purchase-button")>
                        <Button
                            element="a"
                            href=product.sample_url.clone()
                            intention="primary".to_string()
                            target="_blank".to_string()
                        >
                            { "購入する" }
                        </Button>
                    </div>
                    {
                        match &product.material.description {
                            Some(description) => html! {
                                <div class=classes!("ProductDetail-description")>
                                    <Heading
                                        level=HeadingLevel::Two
                                        size=Size::S
                                    >
                                        { "このアイテムについて" }
                                    </Heading>
                                    <Paragraph>
                                        { description.to_string() }
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
