use yew::prelude::*;

pub struct ListTile {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub primary_title: String,
    #[prop_or_default]
    pub secondary_title: Option<String>,
}

impl Component for ListTile {
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
            primary_title,
            secondary_title,
        } = &self.props;

        let classes: Vec<String> = vec!["ncgr-list-tile".to_string()];

        html! {
            <li
                class=classes!(classes, class.clone())
            >
                <div class="ncgr-list-tile__leading">
                    { children.clone() }
                </div>
                <div class="ncgr-list-tile__title">
                    <div class="ncgr-list-tile__primary-title">
                        { primary_title }
                    </div>
                    {
                        match secondary_title {
                            Some(secondary_title) => html! {
                                <div class="ncgr-list-tile__secondary-title">
                                    { secondary_title }
                                </div>
                            },
                            None => html! {
                                <div></div>
                            },
                        }
                    }
                </div>
            </li>
        }
    }
}
