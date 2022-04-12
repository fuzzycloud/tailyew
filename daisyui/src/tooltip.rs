use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub children: Children,
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    html! {
        <div class={classes!("tooltip")}>
            {for props.children.iter()}
        </div>
    }
}
