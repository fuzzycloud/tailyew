use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let tab = html! {
        <div>
        <Tab>{"Who shot first?"}</Tab>
        <Tab tab_classes="tab-active">{"Han Solo"}</Tab>
        <Tab>{"Greedo"}</Tab>
    </div>
    };
    html! {
        <div>
            <Display title="Tab" preview={tab} />
        </div>
    }
}
