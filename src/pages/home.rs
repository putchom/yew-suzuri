use nachiguro::{Col, Container, Heading, Row};
use serde::Deserialize;
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
use crate::models::product::Product;
use crate::components::product_card::ProductCard;

#[derive(Deserialize, Clone)]
pub struct ResponseData {
  products: Vec<Product>,
}

pub enum Msg {
  StartFetch,
  SuccessFetch(ResponseData),
  FailFetch,
}

pub struct Home {
  task: Option<FetchTask>,
  is_loading: bool,
  data: Option<ResponseData>,
  link: ComponentLink<Self>,
  error: Option<String>,
}

impl Component for Home {
  type Message = Msg;
  type Properties = ();

  fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
    link.send_message(Msg::StartFetch);

    Self {
      task: None,
      is_loading: true,
      data: None,
      link,
      error: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> bool {
    match msg {
      Msg::StartFetch => {
        let request = Product::get_product_list("surisurikun");
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

  fn change(&mut self, _props: Self::Properties) -> bool {
    false
  }

  fn view(&self) -> Html {
    html! {
      <div>
        <Container>
          <Heading level=1 size={"m"}>{ "ホーム" }</Heading>
        </Container>
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

impl Home {
  fn success(&self) -> Html {
    match self.data {
      Some(ref res) => {
        html! {
          <Container>
            <Row>
              { for res.products.iter().map( |product|
                html! {
                  <Col col_m={3}>
                    <ProductCard product={product} />
                  </Col>
                })
              }
            </Row>
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