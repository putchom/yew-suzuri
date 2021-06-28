use crate::types::{heading_level::HeadingLevel, size::Size};
use yew::prelude::*;

pub struct Heading {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub level: HeadingLevel,
    #[prop_or_default]
    pub size: Size,
}

impl Component for Heading {
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
        let Props {
            children,
            class,
            level,
            size,
        } = &self.props;

        let classes: Vec<String> = vec!["ncgr-heading".to_string(), format!("-{}", size)];

        html! {
            <@{format!("h{}", level)}
                class=classes!(classes, class.clone())
            >
                { children.clone() }
            </@>
        }
    }
}
