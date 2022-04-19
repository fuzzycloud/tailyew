use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextareaProps {
    pub children: Children,
}

#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    html! {
        <div class={classes!("textarea")}>
        {for props.children.iter()}
        </div>
    }
}
