use nachiguro::Skeleton;
use yew::prelude::*;

pub struct SkeletonProductCard {}

impl Component for SkeletonProductCard {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="ncgr-product-card">
                <Skeleton
                    border_radius="0.5rem".to_string()
                    aspect_ratio="1/1".to_string()
                />
                <div class="ncgr-product-card__info">
                    <Skeleton
                        width="100%".to_string()
                        height="0.875rem".to_string()
                    />
                    <br />
                    <Skeleton
                        width="50%".to_string()
                        height="0.8125rem".to_string()
                    />
                    <br />
                    <Skeleton
                        width="50%".to_string()
                        height="0.8125rem".to_string()
                    />
                </div>
            </div>
        }
    }
}
