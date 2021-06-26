use crate::models::user::User;
use nachiguro::{Avatar, ListTile, ListView};
use yew::prelude::*;

pub struct UserListView {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub user: User,
}

impl Component for UserListView {
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
            <ListView>
                <ListTile primary_title={
                    match user.display_name.clone() {
                        Some(display_name) => display_name,
                        None => user.name.to_string(),
                    }
                }>
                    <Avatar
                        src={
                            match user.avatar_url.clone() {
                                Some(avatar_url) => avatar_url,
                                None => "./icon_default.jpg".to_string(),
                            }
                        }
                        size="m"
                    />
                </ListTile>
            </ListView>
        }
    }
}
