use yew::prelude::*;

pub struct AppBar {
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
    #[prop_or_default]
    pub is_fixed: Option<bool>,
}

impl Component for AppBar {
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
            is_fixed,
        } = &self.props;

        let classes: Vec<String> = vec![
            "ncgr-app-bar".to_string(),
            if *is_active {
                "-active".to_string()
            } else {
                "".to_string()
            },
            match is_fixed {
                Some(_) => "-fixed".to_string(),
                None => "".to_string(),
            },
        ];

        html! {
            <div class=classes!(classes, class.clone())>
                { children.clone() }
            </div>
        }
    }
}
