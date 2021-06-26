use yew::prelude::*;

pub struct ListDivider {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub element: String,
}

impl Component for ListDivider {
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
        let Props { class, element } = &self.props;

        let classes: Vec<String> = vec!["ncgr-list-divider".to_string()];

        html! {
          <@{format!("{}", element)}
            class=classes!(classes, class.clone())
            role="separator"
          />
        }
    }
}
