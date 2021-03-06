mod product_grid_view;
mod skeleton_user_info_view;
mod user_info_view;

use crate::models::user::User;
use nachiguro::Container;
use product_grid_view::ProductGridView;
use serde::Deserialize;
use skeleton_user_info_view::SkeletonUserInfoView;
use user_info_view::UserInfoView;
use yew::{
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Response},
};

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
                    <>
                        <UserInfoView user=res.user.clone() />
                        <Container class=classes!("UserDetail-container")>
                            <ProductGridView user=res.user.clone() />
                        </Container>
                    </>
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
            <>
                <SkeletonUserInfoView />
            </>
        }
    }

    fn fail(&self) -> Html {
        html! {
            <div>{"Fail"}</div>
        }
    }
}
