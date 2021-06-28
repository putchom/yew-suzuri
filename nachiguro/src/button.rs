#![allow(unused_variables)]
use crate::types::size::Size;
use yew::prelude::*;

pub struct Button {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub element: String,
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub is_active: Option<bool>,
    #[prop_or_default]
    pub is_complete: Option<bool>,
    #[prop_or_default]
    pub is_disabled: Option<bool>,
    #[prop_or_default]
    pub is_fluid: Option<bool>,
    #[prop_or_default]
    pub is_light: Option<bool>,
    #[prop_or_default]
    pub is_loading: Option<bool>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub target: Option<String>,
    #[prop_or_default]
    pub intention: Option<String>,
}

impl Component for Button {
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
            element,
            href,
            is_active,
            is_complete,
            is_disabled,
            is_fluid,
            is_light,
            is_loading,
            size,
            target,
            intention,
        } = &self.props;

        let classes: Vec<String> = vec![
            "ncgr-button".to_string(),
            match is_active {
                Some(_) => "ncgr-button--active".to_string(),
                None => "".to_string(),
            },
            match is_complete {
                Some(_) => "ncgr-button--complete".to_string(),
                None => "".to_string(),
            },
            match is_disabled {
                Some(_) => "ncgr-button--disabled".to_string(),
                None => "".to_string(),
            },
            match is_fluid {
                Some(_) => "ncgr-button--fluid".to_string(),
                None => "".to_string(),
            },
            match is_light {
                Some(_) => "ncgr-button--light".to_string(),
                None => "".to_string(),
            },
            match is_loading {
                Some(_) => "ncgr-button--loading".to_string(),
                None => "".to_string(),
            },
            match size {
                Some(size) => format!("ncgr-button--{}", size),
                None => "".to_string(),
            },
            match intention {
                Some(intention) => format!("ncgr-button--{}", intention),
                None => "".to_string(),
            },
        ];

        html! {
            <@{self.props.element.clone()}
                class=classes!(classes, class.clone())
                href=href.to_string()
                target={
                    match self.props.target.clone() {
                        Some(target) => target.to_string(),
                        None => "".to_string()
                    }
                }
            >
                { children.clone() }
            </@>
        }
    }
}
