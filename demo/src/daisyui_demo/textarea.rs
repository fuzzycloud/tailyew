use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let textarea = html! {
        <Textarea>
            {"Bio"}
        </Textarea>
    };
    html! {
        <div>
            <Display title="Textarea" preview={textarea} />
        </div>
    }
}
