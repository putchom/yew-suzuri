mod product_grid_view;
mod user_list_view;

use nachiguro::{Container, Heading};
use product_grid_view::ProductGridView;
use user_list_view::UserListView;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub query: String,
}

pub struct SearchResult {
    props: Props,
}

impl Component for SearchResult {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("Home-page")>
                <Container>
                    <Heading
                        class=classes!("Home-heading")
                        level=1
                        size={"m"}
                    >
                        { "検索結果" }
                    </Heading>
                </Container>
                <ProductGridView query=self.props.query.clone() />
                <UserListView query=self.props.query.clone() />
            </div>
        }
    }
}
