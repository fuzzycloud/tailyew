use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use hero_icons::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let alert = html! {
       <Alert class="alert-info shadow-lg">
    //    <ArrowCircleLeftOutLined />
       <div>
            <span class="mr-16">{"New software update available."} </span>
            <div class="float justify-right items-right">
            <XCircleOutLined />
            </div>


        </div>
       </Alert>
    };
    html! {
        <div>
            <Display title="Alert" preview={alert} />
        </div>
    }
}
