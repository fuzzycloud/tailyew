use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct MenuProps {
    pub children : Children,
    #[prop_or_default]
    pub menu_classes :  &'static str
}

#[function_component(Menu)]
pub fn menu (props:&MenuProps) -> Html {
    html!{
        <ul class={classes!("menu",props.menu_classes)}>
        {for props.children.iter()}
        </ul>
    }
}


