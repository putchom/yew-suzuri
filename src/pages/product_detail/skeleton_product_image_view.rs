use nachiguro::Skeleton;
use yew::prelude::*;

pub struct SkeletonProductImageView {}

impl Component for SkeletonProductImageView {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
          <Skeleton aspect_ratio="1/1".to_string() />
        }
    }
}
