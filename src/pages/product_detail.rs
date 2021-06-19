use nachiguro::{
  Card,
  Col,
  Container,
  Heading,
  Image,
  Paragraph,
  Row,
  TextLink
};
use serde::Deserialize;
use crate::models::Product;
use crate::route::Route;
use num_format::{
  Locale,
  ToFormattedString
};
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
use yew_router::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
  pub id: i32,
}

#[derive(Deserialize, Clone)]
pub struct ResponseData {
  product: Product,
}

pub enum Msg {
  StartFetch,
  SuccessFetch(ResponseData),
  FailFetch,
}

pub struct ProductDetailPage {
  props: Props,
  task: Option<FetchTask>,
  is_loading: bool,
  data: Option<ResponseData>,
  link: ComponentLink<Self>,
  error: Option<String>,
}

impl Component for ProductDetailPage {
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
        let request = Product::get_product_info_by_id(self.props.id);
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
      <div class="ProductDetail-page">
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

impl ProductDetailPage {
  fn success(&self) -> Html {
    type Anchor = RouterAnchor<Route>;

    match self.data {
      Some(ref res) => {
        html! {
          <Container size="l".to_string()>
            <div class="ProductDetail-breadcrumbs">
              <ol class="ncgr-breadcrumbs">
                <li class="ncgr-breadcrumbs__item">
                  <Anchor
                    classes="ncgr-breadcrumbs__link"
                    route=Route::HomePage
                  >
                    <span>
                      { "SUZURI" }
                    </span>
                  </Anchor>
                </li>
                <li class="ncgr-breadcrumbs__item">
                  <a class="ncgr-breadcrumbs__link" href="#">
                    <span>
                      { format!("{}", res.product.item.humanize_name) }
                    </span>
                  </a>
                </li>
                <li class="ncgr-breadcrumbs__item">
                  <a class="ncgr-breadcrumbs__link" href="#">
                    <span>
                      { format!("{}", res.product.material.user.name) }
                    </span>
                  </a>
                </li>
                <li class="ncgr-breadcrumbs__item">
                  <Anchor
                    classes="ncgr-breadcrumbs__link ncgr-breadcrumbs__link--active"
                    route=Route::ProductDetailPage(res.product.id)
                  >
                    <span>
                      { format!("{}", res.product.title) }
                    </span>
                  </Anchor>
                </li>
              </ol>
            </div>
            <Row>
              <Col col_m={7}>
                <Image src=format!("{}", res.product.sample_image_url) />
              </Col>
              <Col col_m={5}>
                <Card class=classes!("ProductDetail-product-card") color="secondary-grouped-background".to_string()>
                  <Container>
                    <Heading level=1 size="m">
                      { format!("{}", res.product.material.title) }
                    </Heading>
                    <Paragraph>
                      <TextLink>
                        { format!("{}", res.product.item.humanize_name) }
                      </TextLink>
                    </Paragraph>
                    <Paragraph>
                      { format!("{}{}", res.product.price_with_tax.to_formatted_string(&Locale::en), "円(税込)") }
                    </Paragraph>
                    {
                      match &res.product.material.description {
                        Some(description) => html! {
                          <div class=classes!("ProductDetail-description")>
                            <Heading level=2 size="s">
                              { "このアイテムについて" }
                            </Heading>
                            <Paragraph>
                              { format!("{}", description) }
                            </Paragraph>
                          </div>
                        },
                        None => html! {
                          <div></div>
                        },
                      }
                    }
                  </Container>
                </Card>
              </Col>
            </Row>
            <Card class=classes!("ProductDetail-user-card")>
              <Container>
                <ul>
                  <li>
                    <div>
                      {
                        match &res.product.material.user.display_name {
                          Some(display_name) => html! {
                            { format!("{}", display_name) }
                          },
                          None => html! {
                            { format!("{}", res.product.material.user.name) }
                          }
                        }
                      }
                    </div>
                  </li>
                </ul>
              </Container>
            </Card>
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