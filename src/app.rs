mod models;

use serde::Deserialize;
use num_format::{Locale, ToFormattedString};
use yew::{
  format::{
    Json,
  },
  prelude::*,
  services::{
    fetch::{
      FetchService,
      FetchTask,
      Response
    }
  }
};

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseData {
  products: Vec<models::product::Product>,
}

#[derive(Debug)]
pub enum Msg {
  StartFetch,
  SuccessFetch(ResponseData),
  FailFetch,
}

#[derive(Debug)]
pub struct App {
  ft: Option<FetchTask>,
  is_loading: bool,
  data: Option<ResponseData>,
  link: ComponentLink<Self>,
  error: Option<String>,
}

impl App {
  fn success(&self) -> Html {
    match self.data {
      Some(ref res) => {
        html! {
          <div>
            { for res.products.iter().map( |product|
              html! {
                <>
                  <a href=format!("{}", product.sample_url)>
                    <img src=format!("{}", product.sample_image_url) />
                  </a>
                  <div>
                    <a href=format!("{}", product.sample_url)>
                      { format!("{}", product.material.title) }
                    </a>
                    <div>
                      { format!("{}", product.item.humanize_name) }
                    </div>
                    <div>
                      { format!("{}{}", product.price_with_tax.to_formatted_string(&Locale::en), "円") }
                    </div>
                  </div>
                </>
              })
            }
          </div>
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

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    link.send_message(Msg::StartFetch);

    Self {
      ft: None,
      is_loading: true,
      data: None,
      link,
      error: None,
    }
  }

  fn rendered(&mut self, _first_render: bool) {
    if _first_render {}
  }

  fn change(&mut self, _props: Self::Properties) -> bool {
    false
  }

  fn update(&mut self, msg: Self::Message) -> bool {
    match msg {
      Msg::StartFetch => {
        let request = models::product::Product::get_product_list("surisurikun");
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
        self.ft = Some(task)
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

  fn view(&self) -> Html {
    html! {
      <div>
        <button onclick=self.link.callback(|_| Msg::StartFetch)>{"Refetch"}</button>
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