use crate::daisyui_demo::display_prop::Display;
use daisyui::input::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let input = html! {
        <Input class="w-full max-w-xs"  input_type={InputTypes::Text} />

    };

    html! {
        <div>
            <Display title="Input" preview={input} />
        </div>
    }
}
