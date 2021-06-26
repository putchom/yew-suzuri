use crate::models::item::Item;
use crate::route::Route;
use nachiguro::{ListTileLeading, ListTileTitle, ListView};
use yew::prelude::*;
use yew_router::prelude::*;

pub struct ItemListView {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub items: Vec<Item>,
}

impl Component for ItemListView {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let Props { items } = &self.props;

        type Anchor = RouterAnchor<Route>;

        html! {
            <ListView>
                { for items.iter().map( |item|
                    html! {
                        // TODO: routeをPropsとしてNachiguroに渡したい
                        <li>
                            <Anchor
                                classes="ncgr-list-tile"
                                route=Route::ItemDetail(item.id)
                            >
                                <ListTileLeading>
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
                                </ListTileLeading>
                                <ListTileTitle primary_title=item.humanize_name.clone() />
                            </Anchor>
                        </li>
                    }
                )}
            </ListView>
        }
    }
}
