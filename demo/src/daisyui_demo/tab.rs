use yew::prelude::*;
use daisyui::prelude::*;


#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
        <div>
        <Tab>{"Who shot first?"}</Tab>
        <Tab tab_classes="tab-active">{"Han Solo"}</Tab>
        <Tab>{"Greedo"}</Tab>
    </div>
    }
}