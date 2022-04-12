use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DropdownProps {
    pub children: Children,
}

#[function_component(Dropdown)]
pub fn dropdown(props: &DropdownProps) -> Html {
    html! {
        <div class={classes!("dropdown")}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownContentProps {
    pub children: Children,
}

#[function_component(DropdownContent)]
pub fn dropdown_content(props: &DropdownContentProps) -> Html {
    html! {
        <div class={classes!("dropdown-content")}>
        {for props.children.iter()}
        </div>
    }
}
