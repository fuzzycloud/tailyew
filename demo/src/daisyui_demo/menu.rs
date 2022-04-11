use yew::prelude::*;
use daisyui::prelude::*;


#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
       <Menu menu_classes="bg-base-100 w-56">
           
                <li><a>{"Item 1"}</a></li>
                <li><a>{"Item 2"}</a></li>
                <li><a>{"Item 3"}</a></li>
           
       </Menu>
    }
}