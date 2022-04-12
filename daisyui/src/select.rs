use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub children: Children,
    #[prop_or_default]
    pub select_classes: &'static str,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    html! {
        <select class={classes!("select",props.select_classes)}>
            {for props.children.iter()}
        </select>
    }
}
