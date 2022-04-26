use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RadioProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Radio)]
pub fn radio(props: &RadioProps) -> Html {
    html! {
        <label class={classes!("label",props.class.clone())}>
        {for props.children.iter()}
        </label>
    }
}

#[derive(Properties, PartialEq)]
pub struct RadioLabelProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(RadioLabel)]
pub fn radio_label(props: &RadioLabelProps) -> Html {
    html! {
        <label class={classes!("label-text",props.class.clone())}>
        {for props.children.iter()}
        </label>
    }
}
