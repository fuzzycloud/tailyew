use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let tooltip = html! {
       <Tooltip>
        <Button text="Button Example" />
       </Tooltip>
    };
    html! {
        <div>
            <Display title="Tooltip" preview={tooltip} />
        </div>
    }
}
