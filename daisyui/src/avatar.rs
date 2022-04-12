use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub children: Children,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    html! {
        <div class={classes!("avatar")}>
        {for props.children.iter()}
        </div>
    }
}
