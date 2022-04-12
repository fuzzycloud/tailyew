use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub children: Children,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    html! {
        <a class={classes!("link")}>
            {for props.children.iter()}
        </a>
    }
}
