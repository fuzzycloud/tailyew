use daisyui::prelude::*;
use hero_icons::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DisplayProp {
    title: &'static str,
    // code : &'static str,
    preview: Html,
}

#[function_component(Display)]
pub fn display(props: &DisplayProp) -> Html {
    html! {
        <div class="m-12">
           <Card card_classes="bg-base-100 shadow-xl">
                <CardBody>
                    <CardTitle>
                        <p> {props.title} </p>
                    </CardTitle>
                    {props.preview.clone()}
                </CardBody>
            </Card>
        </div>
    }
}

#[function_component(Demo)]
pub fn demo() -> Html {
    let alert = html! {

        <Alert color={AlertType::Success}>
        <CheckIconSolid />
        <span>{"Your purchase has been confirmed!"}</span>
        </Alert>
    };

    let alert_type = html! {
        <>
            <Alert color={AlertType::Success} >
            <CheckIconSolid />
                <span>{"Your purchase has been confirmed!"}</span>
            </Alert>
            <Alert color={AlertType::Warning} >
            <CheckIconSolid />
                <span>{"Your purchase has been confirmed!"}</span>
            </Alert>
            <Alert color={AlertType::Info} >
            <CheckIconSolid />
                <span>{"Your purchase has been confirmed!"}</span>
            </Alert>
            <Alert color={AlertType::Error} >
            <CheckIconSolid />
                <span>{"Your purchase has been confirmed!"}</span>
            </Alert>
        </>
    };

    html! {
        <div>
            <Display title="alert" preview={alert} />
            <Display title="Alert Component" preview={alert_type} />
        </div>
    }
}
