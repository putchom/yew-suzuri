use nachiguro::Skeleton;
use yew::prelude::*;

pub struct SkeletonProductCard {
  props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  #[prop_or_default]
  pub class: Classes,
}

impl Component for SkeletonProductCard {
  type Message = ();
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self {
      props,
    }
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    true
  }

  fn view(&self) -> Html {
    let Props {
      class,
    } = &self.props;

    html! {
      <div
        class=classes!(
          "ncgr-product-card",
          class.clone(),
        )
      >
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