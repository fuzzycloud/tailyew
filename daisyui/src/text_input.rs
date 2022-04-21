use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum TextInputTypes {
    Text,
    Number,
    Email,
    Password,
}

impl TextInputTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextInputTypes::Text => "text",
            TextInputTypes::Email => "email",
            TextInputTypes::Number => "number",
            TextInputTypes::Password => "password",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub input_type: TextInputTypes,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,
    #[prop_or_default]
    pub name: &'static str,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    html! {
        <input name={props.name} oninput={props.oninput.clone()} class={classes!("input", props.class.clone())} type={props.input_type.as_str()} />
    }
}
