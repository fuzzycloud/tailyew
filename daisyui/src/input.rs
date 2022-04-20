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
    pub class: Classes,
    #[prop_or_default]
    pub key: &'static str,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub name: &'static str,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <input name={props.name} key={props.key} oninput={props.oninput.clone()} class={classes!("input", props.class.clone())} r#type={props.input_type.as_str()} />
    }
}
