use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let dropdown = html! {
        <div>
            <Dropdown>
                <label tabindex="0" class="btn m-1">{"Click"}</label>
                <div tabindex="0">
                    <DropdownContent class="menu p-2 shadow bg-base-100 rounded-box w-52">
                    <li><a>{"Item 1"}</a></li>
                    <li><a>{"Item 2"}</a></li>
                    </DropdownContent>
                </div>
            </Dropdown>

        </div>
    };

    html! {
        <div>
            <Display title="Dropdown" preview={dropdown} />
        </div>
    }
}
