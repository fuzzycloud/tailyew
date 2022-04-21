use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let badge = html! {
        <>
            <Badge class="badge-secondary">
            {"+99"}
            </Badge>

        </>



    };

    html! {
        <div>
            <Display title="Badge" preview={badge} />
        </div>
    }
}
