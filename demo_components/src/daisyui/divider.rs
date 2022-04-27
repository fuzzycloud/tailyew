use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let divider = html! {
        <div>
            <Divider />
        </div>
    };

    html! {
        <div>
            <Display title="Divider" preview={divider} />
        </div>
    }
}
