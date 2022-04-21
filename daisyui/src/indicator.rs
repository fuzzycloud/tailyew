use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IndicatorProps {
    pub children: Children,
}

#[function_component(Indicator)]
pub fn indicator(props: &IndicatorProps) -> Html {
    html! {
        <div class={classes!("indicator")}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct IndicatorItemProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(IndicatorItem)]
pub fn indicator_item(props: &IndicatorItemProps) -> Html {
    html! {
        <span class={classes!("indicator-item")}>
        {for props.children.iter()}
        </span>
    }
}
