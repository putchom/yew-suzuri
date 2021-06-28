use nachiguro::{
    types::{col_num::ColNum, size::Size},
    Col, Container, Row, Skeleton,
};
use yew::prelude::*;

pub struct SkeletonUserInfoView {}

impl Component for SkeletonUserInfoView {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Skeleton class=classes!("UserDetail-cover") />
                <Container size=Size::M>
                    <Row>
                        <Col col_m=ColNum::Three>
                            <Skeleton class=classes!("UserDetail-avatar") />
                        </Col>
                        <Col col_m=ColNum::Nine>
                            <div class="UserDetail-text">
                                <div>
                                    <Skeleton
                                        class=classes!("skeleton-heading", "-m")
                                        width="8rem".to_string()
                                    />
                                </div>
                                <div>
                                    <Skeleton
                                        class=classes!("skeleton-heading", "-s")
                                        width="4rem".to_string()
                                    />
                                </div>
                            </div>
                        </Col>
                    </Row>
                </Container>
            </>
        }
    }
}
