use yew::prelude::*;

#[derive(PartialEq)]
pub enum InputTypes {
    Text,
    Number,
    Email,
}

impl InputTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputTypes::Text => "text",
            InputTypes::Email => "email",
            InputTypes::Number => "number",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub input_type: InputTypes,
    #[prop_or_default]
    pub input_classes: &'static str,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <input class={classes!("input", props.input_classes)} r#ype={props.input_type.as_str()} />
    }
}
