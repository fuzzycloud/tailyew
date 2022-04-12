use daisyui::input::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
        <Input input_classes="w-full max-w-xs"  input_type={InputTypes::Text} />
    }
}
