use crate::models::item::Item;
use nachiguro::{ListGroup, ListTile, ListView};
use yew::prelude::*;

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

        html! {
            <ListGroup sub_header="アイテムからさがす".to_string()>
                <ListView>
                    { for items.iter().map( |item|
                        html! {
                            <ListTile primary_title=item.humanize_name.clone()>
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
                            </ListTile>
                        }
                    )}
                </ListView>
            </ListGroup>
        }
    }
}
