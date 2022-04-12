use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={classes!("Card")}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardBodyProps {
    pub children: Children,
}

#[function_component(CardBody)]
pub fn card_body(props: &CardBodyProps) -> Html {
    html! {
        <div class={classes!("HelloBody")}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardTitleProps {
    pub children: Children,
}

#[function_component(CardTitle)]
pub fn card_title(props: &CardBodyProps) -> Html {
    html! {
        <div class={classes!("Hello-title")}>
            {for props.children.iter()}
        </div>
    }
}
