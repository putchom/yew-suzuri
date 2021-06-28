mod item_list_view;
mod skeleton_item_list_view;

use crate::models::item::Item;
use crate::route::Route;
use item_list_view::ItemListView;
use nachiguro::{
    types::{col_num::ColNum, size::Size},
    Col, Container, ListGroup, Row,
};
use serde::Deserialize;
use skeleton_item_list_view::SkeletonItemListView;
use yew::{
    events::InputData,
    format::Json,
    prelude::*,
    services::fetch::{FetchService, FetchTask, Response},
};
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

#[derive(Deserialize, Clone)]
pub struct ResponseData {
    items: Vec<Item>,
}

pub enum Msg {
    StartFetch,
    SuccessFetch(ResponseData),
    FailFetch,
    OnInput(String),
}
pub struct Search {
    task: Option<FetchTask>,
    is_loading: bool,
    input_value: String,
    data: Option<ResponseData>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Component for Search {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::StartFetch);

        Self {
            task: None,
            is_loading: true,
            input_value: "".to_string(),
            data: None,
            link,
            error: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::StartFetch => {
                let request = Item::get_item_list();
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
            Msg::OnInput(value) => {
                self.input_value = value;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
            <div class=classes!("Search-page")>
                <Container is_gapless=true size=Size::M>
                    <Container class=classes!("Search-textfield-container")>
                        <Row>
                            <Col col=ColNum::Nine>
                                // TODO: Textfieldにoninputわたせるようにする
                                <div class="ncgr-textfield">
                                    <label class="ncgr-textfield__label" for="query">
                                        { "キーワードからさがす" }
                                    </label>
                                    <input
                                        class="ncgr-textfield__input"
                                        id="query"
                                        name="query"
                                        type="text" value={self.input_value.clone()}
                                        oninput=self.link.callback(|event: InputData| Msg::OnInput(event.value))
                                        placeholder="例）犬, 忍者スリスリくん"
                                    />
                                </div>
                            </Col>
                            <Col col=ColNum::Three>
                                // TODO: ButtonでAnchorがつかえるようにする
                                <Anchor
                                    classes="ncgr-button ncgr-button--primary"
                                    route=Route::SearchResult(self.input_value.clone())
                                >
                                    { "さがす" }
                                </Anchor>
                            </Col>
                        </Row>
                    </Container>
                    <ListGroup sub_header="カテゴリーからさがす".to_string()>
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
                    </ListGroup>
                </Container>
            </div>
        }
    }
}

impl Search {
    fn success(&self) -> Html {
        match self.data {
            Some(ref res) => {
                html! {
                    <ItemListView items=res.items.clone() />
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
            <SkeletonItemListView />
        }
    }

    fn fail(&self) -> Html {
        html! {
            <div>{"Fail"}</div>
        }
    }
}
