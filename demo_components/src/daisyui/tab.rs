use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let tab = html! {
        <Tabs class="text-3xl">
            <Tab>{"Who shot first?"}</Tab>
            <Tab class="tab-active">{"Han Solo"}</Tab>
            <Tab>{"Greedo"}</Tab>
        </Tabs>
    };
    html! {
        <div>
            <Display title="Tab" preview={tab} />
        </div>
    }
}
