use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let steps = html! {
        <Steps>
        <Step step_classes="step-primary">{"Who shot first?"}</Step>
        <Step step_classes="step-primary">{"Han Solo"}</Step>
        <Step>{"Greedo"}</Step>
    </Steps>
    };
    html! {
        <div>
            <Display title="Steps" preview={steps} />
        </div>
    }
}
