use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let menu = html! {
       <Menu class="bg-base-100 w-56">

                <li><a>{"Item 1"}</a></li>
                <li><a>{"Item 2"}</a></li>
                <li><a>{"Item 3"}</a></li>

       </Menu>
    };
    html! {
        <div>
            <Display title="Menu" preview={menu} />
        </div>
    }
}
