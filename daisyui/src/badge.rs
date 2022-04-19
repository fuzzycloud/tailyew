use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    html! {
        <div class={classes!("badge",props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}
