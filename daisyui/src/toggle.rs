use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub children: Children,
    #[prop_or_default]
    pub label_classes: &'static str,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    html! {
        <div class={classes!("label",props.label_classes)}>
            {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LabelTextProps {
    pub children: Children,
    #[prop_or_default]
    pub label_text_classes: &'static str,
}

#[function_component(LabelText)]
pub fn label_text(props: &LabelTextProps) -> Html {
    html! {
        <div class={classes!("label-text",props.label_text_classes)}>
            {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ToggleProps {
    pub children: Children,
}

#[function_component(Toggle)]
pub fn toggle(props: &ToggleProps) -> Html {
    html! {
        <div class={classes!({"toggle"})}>
            {for props.children.iter()}
        </div>
    }
}

#[derive(PartialEq)]
pub enum InputTypes {
    Checkbox,
}

impl InputTypes {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputTypes::Checkbox => "checkbox",
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
      <input class={classes!("checkbox", props.input_classes)} r#ype={props.input_type.as_str()} />
    }
}
