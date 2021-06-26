use yew::prelude::*;

pub struct Col {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub col: Option<i32>,
    #[prop_or_default]
    pub col_s: Option<i32>,
    #[prop_or_default]
    pub col_m: Option<i32>,
    #[prop_or_default]
    pub col_l: Option<i32>,
    #[prop_or_default]
    pub offset_s: Option<i32>,
    #[prop_or_default]
    pub offset_m: Option<i32>,
    #[prop_or_default]
    pub offset_l: Option<i32>,
}

impl Component for Col {
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
            col,
            col_s,
            col_m,
            col_l,
            offset_s,
            offset_m,
            offset_l,
        } = &self.props;

        let classes: Vec<String> = vec![
            "ncgr-col".to_string(),
            match col {
                Some(num) => format!("-col-{}", num),
                None => "".to_string(),
            },
            match col_s {
                Some(num) => format!("-col-s-{}", num),
                None => "".to_string(),
            },
            match col_m {
                Some(num) => format!("-col-m-{}", num),
                None => "".to_string(),
            },
            match col_l {
                Some(num) => format!("-col-l-{}", num),
                None => "".to_string(),
            },
            match offset_s {
                Some(num) => format!("-offset-s-{}", num),
                None => "".to_string(),
            },
            match offset_m {
                Some(num) => format!("-offset-m-{}", num),
                None => "".to_string(),
            },
            match offset_l {
                Some(num) => format!("-offset-l-{}", num),
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
