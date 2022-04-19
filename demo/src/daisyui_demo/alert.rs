use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use hero_icons::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let alert = html! {
        <div >
        <Alert color={AlertType::Success}>
        <CheckIconSolid />
        <span>{"Your purchase has been confirmed!"}</span>
        </Alert>
        </div>
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
