use crate::components::{product_card::ProductCard, skeleton_product_card::SkeletonProductCard};
use crate::models::product::Product;
use nachiguro::{Col, Container, Heading, Row, Skeleton};
use serde::Deserialize;
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
    products: Vec<Product>,
}

pub enum Msg {
    StartFetch,
    SuccessFetch(ResponseData),
    FailFetch,
}

pub struct ItemDetail {
    props: Props,
    task: Option<FetchTask>,
    is_loading: bool,
    data: Option<ResponseData>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Component for ItemDetail {
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
                let request = Product::get_product_list_by_item_id(self.props.id);
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
          <div class="ItemDetail-page">
            <Container>
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
            </Container>
          </div>
        }
    }
}

impl ItemDetail {
    fn success(&self) -> Html {
        match self.data {
            Some(ref res) => {
                html! {
                  <>
                    <Heading
                      class=classes!("ItemDetail-heading")
                      level=1
                      size={"m"}
                    >
                      { res.products[0].item.humanize_name.to_string() }
                    </Heading>
                    <Row>
                      { for res.products.iter().map( |product|
                        html! {
                          <Col col={6} col_m={4} col_l={2}>
                            <ProductCard product={product} />
                          </Col>
                        })
                      }
                    </Row>
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
        let dummy_product_list: Vec<i32> = (0..12).collect();
        html! {
          <>
            <Heading
              class=classes!("ItemDetail-heading")
              level=1
              size={"m"}
            >
              <Skeleton
                class=classes!("skeleton-heading", "-m")
                width="8rem".to_string()
              />
            </Heading>
            <Row>
              { for dummy_product_list.iter().map( |_|
                html! {
                  <Col col={6} col_m={4} col_l={2}>
                    <SkeletonProductCard />
                  </Col>
                }
              )}
            </Row>
          </>
        }
    }

    fn fail(&self) -> Html {
        html! {
          <div>{"Fail"}</div>
        }
    }
}
