use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class={classes!("card", props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardBodyProps {
    pub children: Children,
    pub class: Classes,
}

#[function_component(CardBody)]
pub fn card_body(props: &CardBodyProps) -> Html {
    html! {
        <div class={classes!("card-body",props.class.clone())}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardTitleProps {
    pub children: Children,
    pub class: Classes,
}

#[function_component(CardTitle)]
pub fn card_title(props: &CardTitleProps) -> Html {
    html! {
        <div class={classes!("card-title",props.class.clone())}>
            {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardActionsProps {
    pub children: Children,
}

#[function_component(CardActions)]
pub fn card_actions(props: &CardActionsProps) -> Html {
    html! {
        <div class={classes!("card-actions")}>
            {for props.children.iter()}
        </div>
    }
}
