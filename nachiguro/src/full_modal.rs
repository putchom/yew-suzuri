use crate::app_bar::AppBar;
use yew::prelude::*;

pub struct FullModal {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub is_active: bool,
}

impl Component for FullModal {
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
            is_active,
        } = &self.props;

        let classes: Vec<String> = vec![
            "ncgr-full-modal".to_string(),
            if *is_active {
                "-active".to_string()
            } else {
                "".to_string()
            },
        ];

        html! {
            <div
                class=classes!(classes, class.clone())
            >
                <AppBar is_active=true>
                { "さがす" }
                </AppBar>
                <div class="ncgr-full-modal__body">
                { children.clone() }
                </div>
            </div>
        }
    }
}
