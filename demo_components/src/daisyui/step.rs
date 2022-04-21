use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let steps = html! {
        <Steps>
        <Step class="step-primary">{"Who shot first?"}</Step>
        <Step class="step-primary">{"Han Solo"}</Step>
        <Step>{"Greedo"}</Step>
    </Steps>
    };
    html! {
        <div>
            <Display title="Steps" preview={steps} />
        </div>
    }
}
