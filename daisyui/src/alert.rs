use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
        <div class={classes!("alert",props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}
