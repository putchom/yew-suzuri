use nachiguro::{ListTile, ListTileLeading, ListView, Skeleton};
use yew::prelude::*;

pub struct SkeletonItemListView {}

impl Component for SkeletonItemListView {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let dummy_item_list: Vec<i32> = (0..12).collect();

        html! {
            <ListView>
                { for dummy_item_list.iter().map( |_|
                    html! {
                        <li>
                            <ListTile>
                                <ListTileLeading>
                                    <Skeleton
                                        width="40px".to_string()
                                        height="40px".to_string()
                                        border_radius="50%".to_string()
                                    />
                                </ListTileLeading>
                                <Skeleton
                                    width="50%".to_string()
                                    height="1rem".to_string()
                                />
                            </ListTile>
                        </li>
                    }
                )}
            </ListView>
        }
    }
}
