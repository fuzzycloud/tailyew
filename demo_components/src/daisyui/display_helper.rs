use daisyui::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DisplayProp {
    pub title: &'static str,
    pub preview: Html,
}

#[function_component(Display)]
pub fn display(props: &DisplayProp) -> Html {
    html! {
        <div class="m-12 p-12">
           <Card class="bg-base-100 shadow-xl">
                <CardBody class="">
                    <CardTitle class="">
                        <p> {props.title} </p>
                    </CardTitle>
                    {props.preview.clone()}
                </CardBody>
            </Card>
        </div>
    }
}
