#![allow(clippy::large_enum_variant)]

mod breadcrumbs;
mod product_image_view;
mod product_info_card;
mod skeleton_breadcrumbs;
mod skeleton_product_image_view;
mod skeleton_product_info_card;
mod user_card;

use crate::models::product::Product;
use breadcrumbs::Breadcrumbs;
use nachiguro::{Col, Container, Row};
use product_image_view::ProductImageView;
use product_info_card::ProductInfoCard;
use serde::Deserialize;
use skeleton_breadcrumbs::SkeletonBreadcrumbs;
use skeleton_product_image_view::SkeletonProductImageView;
use skeleton_product_info_card::SkeletonProductInfoCard;
use user_card::UserCard;
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
    product: Product,
}

pub enum Msg {
    StartFetch,
    SuccessFetch(ResponseData),
    FailFetch,
}

pub struct ProductDetail {
    props: Props,
    task: Option<FetchTask>,
    is_loading: bool,
    data: Option<ResponseData>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Component for ProductDetail {
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
            <div class="ProductDetail-page">
                <Container is_gapless=true size="l".to_string()>
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

impl ProductDetail {
    fn success(&self) -> Html {
        match self.data {
            Some(ref res) => {
                html! {
                    <>
                        <Breadcrumbs product=res.product.clone() />
                        <Row is_gapless=true>
                            <Col col_m={7}>
                                <ProductImageView product=res.product.clone() />
                            </Col>
                            <Col col_m={5}>
                                <div class="ProductDetail-product-info-card">
                                    <ProductInfoCard product=res.product.clone() />
                                </div>
                            </Col>
                        </Row>
                        <UserCard user=res.product.material.user.clone() />
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
                <SkeletonBreadcrumbs />
                <Row is_gapless=true>
                    <Col col_m={7}>
                        <SkeletonProductImageView />
                    </Col>
                    <Col col_m={5}>
                        <div class="ProductDetail-product-info-card">
                            <SkeletonProductInfoCard />
                        </div>
                    </Col>
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
