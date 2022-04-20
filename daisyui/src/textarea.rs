use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    html! {
        <textarea class={classes!("textarea",props.class.clone())}>
        {for props.children.iter()}
        </textarea>
    }
}
