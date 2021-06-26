#![allow(unused_variables)]

use yew::prelude::*;

pub struct Textfield {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub input_type: String,
    #[prop_or_default]
    pub label_text: Option<String>,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub placeholder: Option<String>,
}

impl Component for Textfield {
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
            class,
            input_type,
            label_text,
            name,
            placeholder,
        } = &self.props;

        let textfield_classes: Vec<String> = vec!["ncgr-textfield".to_string()];

        let input_classes: Vec<String> = vec!["ncgr-textfield__input".to_string()];

        html! {
            <div class=classes!(textfield_classes, class.clone())>
                {
                    match label_text {
                        Some(label_text) => html! {
                            <label class="ncgr-textfield__label" for=name.to_string()>
                                { label_text }
                            </label>
                        },
                        None => html! {
                            <div></div>
                        }
                    }
                }
                <input
                    class=classes!(input_classes)
                    id=name.to_string()
                    name=name.to_string()
                    type=input_type.to_string()
                    placeholder={
                        match self.props.placeholder.clone() {
                            Some(placeholder) => placeholder,
                            None => "".to_string(),
                        }
                    }
                />
            </div>
        }
    }
}
