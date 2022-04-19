use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    html! {
        <div class={classes!("dropdown",props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownContentProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DropdownContent)]
pub fn dropdown_content(props: &DropdownContentProps) -> Html {
    html! {
        <ul class={classes!("dropdown-content",props.class.clone())}>
        {for props.children.iter()}
        </ul>
    }
}
