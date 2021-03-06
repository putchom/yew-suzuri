use crate::types::size::Size;
use yew::prelude::*;

pub struct Container {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub is_gapless: Option<bool>,
    #[prop_or_default]
    pub size: Option<Size>,
}

impl Component for Container {
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
            is_gapless,
            size,
        } = &self.props;

        let classes: Vec<String> = vec![
            "ncgr-container".to_string(),
            match is_gapless {
                Some(_) => "-no-gaps".to_string(),
                None => "".to_string(),
            },
            match size {
                Some(size) => format!("-{}", size),
                None => "".to_string(),
            },
        ];

        html! {
            <div
                class=classes!(classes, class.clone())
            >
                { children.clone() }
            </div>
        }
    }
}
