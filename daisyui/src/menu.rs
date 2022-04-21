use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MenuProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    html! {
        <ul class={classes!("menu",props.class.clone())}>
        {for props.children.iter()}
        </ul>
    }
}
