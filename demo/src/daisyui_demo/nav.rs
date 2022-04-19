use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let nav = html! {
        <Nav class="bg-base-100">
            <div class="flex-1">
                <Button text="TailYew" />
             </div>
            <div class="flex-none">
                <Menu menu_classes="menu-horizontal p-0">

                    <li><a>{"Home"}</a></li>
                    <li><a>{"Svg"}</a></li>
                    <li><a>{"Item 3"}</a></li>

                </Menu>
            </div>
        </Nav>
    };
    html! {
        <div>
            <Display title="Nav" preview={nav} />
        </div>
    }
}
