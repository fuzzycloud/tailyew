use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let tab = html! {
        <div>
        <Tab>{"Who shot first?"}</Tab>
        <Tab class="tab-active">{"Han Solo"}</Tab>
        <Tab>{"Greedo"}</Tab>
    </div>
    };
    html! {
        <div>
            <Display title="Tab" preview={tab} />
        </div>
    }
}
