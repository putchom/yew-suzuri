use nachiguro::{
    types::{background_color::BackgroundColor, heading_level::HeadingLevel, size::Size},
    Card, Container, Heading, Paragraph, Skeleton,
};
use yew::prelude::*;

pub struct SkeletonProductInfoCard {}

impl Component for SkeletonProductInfoCard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
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
                        <Skeleton
                            class=classes!("skeleton-heading", "-m")
                            width="50%".to_string()
                        />
                    </Heading>
                    <Paragraph>
                        <Skeleton
                            width="8rem".to_string()
                            height="1rem".to_string()
                        />
                    </Paragraph>
                    <Paragraph>
                        <Skeleton
                            width="6rem".to_string()
                            height="1rem".to_string()
                        />
                    </Paragraph>
                </Container>
            </Card>
        }
    }
}
