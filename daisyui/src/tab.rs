use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    pub children: Children,
    pub class: Classes,
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    html! {
        <div class={classes!("tabs",props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Tab)]
pub fn tab(props: &TabProps) -> Html {
    html! {
        <div class={classes!("tab",props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}
