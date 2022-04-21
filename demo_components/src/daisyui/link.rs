use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let link = html! {
       <Link>{"I'm a simple link"}
       </Link>
    };
    html! {
        <div>
            <Display title="Link" preview={link} />
        </div>
    }
}
