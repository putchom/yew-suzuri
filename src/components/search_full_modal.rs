use nachiguro::{
  FullModal,
  FullModalContents,
  ListGroup,
  ListTile,
  ListView,
};
use serde::Deserialize;
use yew::{
  format::Json,
  prelude::*,
  services::fetch::{
    FetchService,
    FetchTask,
    Response
  }
};
use crate::models::item::Item;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub is_active: bool,
}

#[derive(Deserialize, Clone)]
pub struct ResponseData {
  items: Vec<Item>,
}

pub enum Msg {
  StartFetch,
  SuccessFetch(ResponseData),
  FailFetch,
}
pub struct SearchFullModal {
  props: Props,
  task: Option<FetchTask>,
  is_loading: bool,
  data: Option<ResponseData>,
  link: ComponentLink<Self>,
  error: Option<String>,
}

impl Component for SearchFullModal {
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
        let request = Item::get_item_list();
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
      <FullModal is_active=self.props.is_active>
        <FullModalContents is_active=true>
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
        </FullModalContents>
      </FullModal>
    }
  }
}

impl SearchFullModal {
  fn success(&self) -> Html {
    match self.data {
      Some(ref res) => {
        html! {
          <ListGroup sub_header="アイテムからさがす".to_string()>
            <ListView>
              { for res.items.iter().map( |item|
                html! {
                  <ListTile primary_title=item.humanize_name.clone()>
                    <img
                      src={
                        match item.icon_urls.get("png") {
                          Some(url) => url.to_string(),
                          None => "".to_string()
                        }
                      }
                      width="40px"
                      height="40px"
                    />
                  </ListTile>
                }
              )}
            </ListView>
          </ListGroup>
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
      <>{"fetching..."}</>
    }
  }

  fn fail(&self) -> Html {
    html! {
      <div>{"Fail"}</div>
    }
  }
}