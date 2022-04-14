use daisyui::prelude::*;
use hero_icons::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {

        <Alert color={AlertType::Success}>
        <CheckIconSolid />
        <span>{"Your purchase has been confirmed!"}</span>
        </Alert>

    }
}
