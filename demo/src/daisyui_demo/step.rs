use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
        <Steps>
        <Step step_classes="step-primary">{"Who shot first?"}</Step>
        <Step step_classes="step-primary">{"Han Solo"}</Step>
        <Step>{"Greedo"}</Step>
    </Steps>
    }
}
