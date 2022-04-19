use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html! {
        <div class={classes!("modal",props.classes.clone())}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ModalBoxProps {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(ModalBox)]
pub fn modal_box(props: &ModalBoxProps) -> Html {
    html! {
        <div class={classes!("modal-box",props.classes.clone())}>
        {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ModalActionProps {
    pub children: Children,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(ModalAction)]
pub fn modal_action(props: &ModalActionProps) -> Html {
    html! {
        <div class={classes!("modal-action",props.classes.clone())}>
        {for props.children.iter()}
        </div>
    }
}
