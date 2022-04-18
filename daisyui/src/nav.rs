use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    html! {
        <div class={classes!(
            "navbar",
            props.class.clone()
        )}>
            {for props.children.iter()}
        </div>
    }
}
