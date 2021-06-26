use crate::components::header::Header;
use crate::pages::{
    home::Home, item_detail::ItemDetail, product_detail::ProductDetail, search::Search,
    user_detail::UserDetail,
};
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render = Router::render(move |switch: Route| match switch {
            Route::ItemDetail(id) => {
                html! { <ItemDetail id=id /> }
            }
            Route::ProductDetail(id) => {
                html! { <ProductDetail id=id /> }
            }
            Route::Search => {
                html! { <Search /> }
            }
            Route::UserDetail(id) => {
                html! { <UserDetail id=id /> }
            }
            Route::Home => {
                html! { <Home /> }
            }
        });

        html! {
            <>
                <Header />
                <Router<Route, ()> render=render />
            </>
        }
    }
}
