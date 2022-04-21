use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let text_input = html! {
        <TextInput class="w-full max-w-xs"  input_type={TextInputTypes::Text} />
    };

    let password = html! {
        <TextInput key={"some_password"} class="w-full max-w-xs" input_type={TextInputTypes::Password} />
    };

    html! {
        <>
            <Display title="Input" preview={text_input} />
            <Display title="Password" preview={password} />
        </>
    }
}
