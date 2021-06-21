use nachiguro::{
  Avatar,
  Card,
  Container,
};
use crate::models::User;
use crate::route::Route;

use yew::prelude::*;
use yew_router::prelude::*;

pub struct UserCard {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub user: User,
}

impl Component for UserCard {
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
      user,
    } = &self.props;

    type Anchor = RouterAnchor<Route>;

    html! {
      <Card class=classes!("user-card")>
        <Container>
          <ul>
            <li>
              <div>
                <Anchor
                  route=Route::UserDetail(user.id)
                >
                  <Avatar
                    src={ format!("{}",
                      match &user.avatar_url {
                        Some(avatar_url) => avatar_url,
                        None => "./icon_default.jpg"
                      }
                    )}
                    size="l"
                  />
                  {
                    match &user.display_name {
                      Some(display_name) => html! {
                        { format!("{}", display_name) }
                      },
                      None => html! {
                        { format!("{}", user.name) }
                      }
                    }
                  }
                </Anchor>
              </div>
            </li>
          </ul>
        </Container>
      </Card>
    }
  }
}