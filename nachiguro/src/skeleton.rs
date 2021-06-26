use yew::prelude::*;

pub struct Skeleton {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub aspect_ratio: Option<String>,
    #[prop_or_default]
    pub border_radius: Option<String>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub height: Option<String>,
    #[prop_or_default]
    pub width: Option<String>,
}

impl Component for Skeleton {
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
            aspect_ratio,
            border_radius,
            class,
            height,
            width,
        } = &self.props;

        let classes: Vec<String> = vec!["skeleton".to_string()];

        let styles: Vec<String> = vec![
            match aspect_ratio {
                Some(aspect_ratio) => format!("aspect-ratio: {};", aspect_ratio),
                None => "".to_string(),
            },
            match border_radius {
                Some(border_radius) => format!("border-radius: {};", border_radius),
                None => "".to_string(),
            },
            match height {
                Some(height) => format!("height: {};", height),
                None => "".to_string(),
            },
            match width {
                Some(width) => format!("width: {};", width),
                None => "".to_string(),
            },
        ];

        html! {
            <div
                class=classes!(classes, class.clone())
                style=styles.join(" ")
            />
        }
    }
}
