use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    html! {
        <select class={classes!("select",props.class.clone())}>
            {for props.children.iter()}
        </select>
    }
}
