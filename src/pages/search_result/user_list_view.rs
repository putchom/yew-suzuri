use crate::models::user::User;
use nachiguro::Container;
use serde::Deserialize;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Response},
};

#[derive(Properties, Clone)]
pub struct Props {
    pub query: String,
}

#[derive(Deserialize, Clone)]
pub struct ResponseData {
    users: Vec<User>,
}

pub enum Msg {
    StartFetch,
    SuccessFetch(ResponseData),
    FailFetch,
}

pub struct UserListView {
    props: Props,
    task: Option<FetchTask>,
    is_loading: bool,
    data: Option<ResponseData>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Component for UserListView {
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
                let request = User::get_user_list_by_name(self.props.query.clone());
                let callback = self.link.callback(
                    |response: Response<Json<Result<ResponseData, anyhow::Error>>>| {
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
            <Container>
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
            </Container>
        }
    }
}

impl UserListView {
    fn success(&self) -> Html {
        match self.data {
            Some(ref res) => {
                html! {
                    { for res.users.iter().map( |user|
                        html! {
                            { user.name.to_string() }
                        }
                    )}
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
