use crate::models::user::User;
use nachiguro::{
    types::{col_num::ColNum, size::Size},
    Col, Container, Heading, Row,
};
use yew::prelude::*;

pub struct UserInfoView {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub user: User,
}

impl Component for UserInfoView {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
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
        let Props { user } = &self.props;

        html! {
            <>
                <div class="UserDetail-cover"></div>
                <Container size=Size::M>
                    <Row>
                        <Col col_m=ColNum::Three>
                            <img
                                class="UserDetail-avatar"
                                src={
                                    match user.avatar_url.clone() {
                                        Some(avatar_url) => avatar_url,
                                        None => "/images/icon_default.jpg".to_string()
                                    }
                                }
                            />
                        </Col>
                        <Col col_m=ColNum::Nine>
                            <div class="UserDetail-text">
                                <div>
                                    <Heading size=Size::M>
                                        {
                                            match user.display_name.clone() {
                                                Some(display_name) => display_name,
                                                None => user.name.to_string()
                                            }
                                        }
                                    </Heading>
                                </div>
                                <div>
                                    <Heading
                                        size=Size::S
                                        class=classes!("ncgr-typography-secondary-label")
                                    >
                                        { user.name.to_string() }
                                    </Heading>
                                </div>
                            </div>
                        </Col>
                    </Row>
                </Container>
            </>
        }
    }
}
