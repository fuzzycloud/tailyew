use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let input = html! {
        <TextInput class="w-full max-w-xs"  input_type={TextInputTypes::Text} />
    };

    html! {
        <div>
            <Display title="Input" preview={input} />
        </div>
    }
}
