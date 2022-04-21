use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let textarea = html! {
        <Textarea class="textarea">
        </Textarea>
    };
    html! {
        <div>
            <Display title="Textarea" preview={textarea} />
        </div>
    }
}
