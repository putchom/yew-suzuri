use nachiguro::Skeleton;
use yew::prelude::*;

pub struct SkeletonBreadcrumbs {}

impl Component for SkeletonBreadcrumbs {
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
          <div class="ProductDetail-breadcrumbs">
            <Skeleton
              width="10rem".to_string()
              height="0.75rem".to_string()
            />
          </div>
        }
    }
}
