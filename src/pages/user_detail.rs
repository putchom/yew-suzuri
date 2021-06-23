use nachiguro::{
  Avatar,
  Container,
};
use serde::Deserialize;
use yew::{
  format::Json,
  prelude::*,
  services::fetch::{
    FetchService,
    FetchTask,
    Response,
  }
};
use crate::models::user::User;

#[derive(Properties, Clone)]
pub struct Props {
  pub id: i32,
}

#[derive(Deserialize, Clone)]
pub struct ResponseData {
  user: User,
}

pub enum Msg {
  StartFetch,
  SuccessFetch(ResponseData),
  FailFetch,
}

pub struct UserDetail {
  props: Props,
  task: Option<FetchTask>,
  is_loading: bool,
  data: Option<ResponseData>,
  link: ComponentLink<Self>,
  error: Option<String>,
}

impl Component for UserDetail {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    link.send_message(Msg::StartFetch);

    Self {
      props,
      task: None,
      is_loading: true,
      data: None,
      link,
      error: None,
    }
  }

  fn update(&mut self, message: Self::Message) -> ShouldRender {
    match message {
      Msg::StartFetch => {
        let request = User::get_user_info_by_id(self.props.id);
        let callback = self.link.callback(|response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
            let Json(data) = response.into_body();

            match data {
              Ok(data) => Msg::SuccessFetch(data),
              Err(_) => Msg::FailFetch,
            }
          },
        );
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        self.is_loading = true;
        self.task = Some(task)
      }
      Msg::SuccessFetch(response) => {
        self.is_loading = false;
        self.data = Some(response);
      }
      Msg::FailFetch => {
        self.error = Some("error".to_string());
        self.is_loading = false;
      }
    }
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div class="UserDetail-page">
        // <button onclick=self.link.callback(|_| Msg::StartFetch)>{"Refetch"}</button>
        {
          match (self.is_loading, self.data.as_ref(), self.error.as_ref()) {
            (true, _, _) => {
              self.fetching()
            }
            (false, Some(_), None) => {
              self.success()
            }
            (false, None, None) => {
              self.fail()
            }
            (_, _, _) => {
              self.fail()
            }
          }
        }
      </div>
    }
  }
}

impl UserDetail {
  fn success(&self) -> Html {
    match self.data {
      Some(ref res) => {
        html! {
          <Container>
            <Avatar
              src={
                match res.user.avatar_url.clone() {
                  Some(avatar_url) => avatar_url,
                  None => "/images/icon_default.jpg".to_string()
                }
              }
              size="l"
            />
            { res.user.name.to_string() }
          </Container>
        }
      }
      None => {
        html! {
          <>{"None"}</>
        }
      }
    }
  }

  fn fetching(&self) -> Html {
    html! {
      <div>{"Fetching..."}</div>
    }
  }

  fn fail(&self) -> Html {
    html! {
      <div>{"Fail"}</div>
    }
  }
}