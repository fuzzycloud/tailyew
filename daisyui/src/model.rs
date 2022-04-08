use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct MyModalProps {
    pub children : Children
}

#[function_component(MyModal)]
pub fn my_modal(props:&MyModalProps) -> Html{

    html!{
        <div class={classes!("Hello-Modal")}>
        {for props.children.iter()}
        </div>
    }

}

#[derive(Properties,PartialEq)]
pub struct ModalBoxProps{
    pub children : Children
}

#[function_component(ModalBox)]
pub fn modal_box(props:&ModalBoxProps) -> Html {
    html! {
        <div class={classes!("hello-box")}>
        {for props.children.iter()}
        </div>
    }
}