use crate::models::user::User;
use nachiguro::{Avatar, Col, Container, Heading, Row};
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
                <Container size="m".to_string()>
                    <Row>
                        <Col col_m={3}>
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
                        <Col col_m={9}>
                            <div class="UserDetail-text">
                                <Heading size="m".to_string()>
                                    {
                                        match user.display_name.clone() {
                                            Some(display_name) => display_name,
                                            None => user.name.to_string()
                                        }
                                    }
                                </Heading>
                            </div>
                        </Col>
                    </Row>
                </Container>
            </>
        }
    }
}
