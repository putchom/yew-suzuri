use crate::models::product::Product;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Breadcrumbs {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub product: Product,
}

impl Component for Breadcrumbs {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let Props { product } = &self.props;

        type Anchor = RouterAnchor<Route>;

        html! {
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
                        <Anchor
                            classes="ncgr-breadcrumbs__link"
                            route=Route::ItemDetail(product.item.id)
                        >
                            <span>
                                { product.item.humanize_name.to_string() }
                            </span>
                        </Anchor>
                    </li>
                    <li class="ncgr-breadcrumbs__item">
                        <Anchor
                            classes="ncgr-breadcrumbs__link"
                            route=Route::UserDetail(product.material.user.id)
                        >
                            <span>
                                { product.material.user.name.to_string() }
                            </span>
                        </Anchor>
                    </li>
                    <li class="ncgr-breadcrumbs__item">
                        <Anchor
                            classes="ncgr-breadcrumbs__link ncgr-breadcrumbs__link--active"
                            route=Route::ProductDetail(product.id)
                        >
                            <span>
                                { product.title.to_string() }
                            </span>
                        </Anchor>
                    </li>
                </ol>
            </div>
        }
    }
}
