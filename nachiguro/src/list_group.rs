use yew::prelude::*;

pub struct ListGroup {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub sub_header: Option<String>,
}

impl Component for ListGroup {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
            sub_header,
        } = &self.props;

        let classes: Vec<String> = vec!["ncgr-list-group".to_string()];

        html! {
            <section
                class=classes!(classes, class.clone())
            >
                {
                    match sub_header {
                        Some(sub_header) => html! {
                            <div class="ncgr-list-group__subheader">
                                { sub_header }
                            </div>
                        },
                        None => html! {
                            <div></div>
                        },
                    }
                }
                { children.clone() }
            </section>
        }
    }
}
