use crate::components::logo::Logo;
use crate::route::Route;
use nachiguro::AppBar;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
            <div class="Header">
                <AppBar is_active=true>
                    <Anchor
                        classes="Header-logo"
                        route=Route::HomePage
                    >
                        <Logo />
                    </Anchor>
                </AppBar>
            </div>
        }
    }
}
