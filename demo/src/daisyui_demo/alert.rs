use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {

        <Alert text="Hello" color={AlertType::AlertSuccess}>
        </Alert>

    }
}
