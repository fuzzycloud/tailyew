use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct StepsProps {
    pub children : Children,
}

#[function_component(Steps)]
pub fn steps (props:&StepsProps) -> Html {
    html!{
        <ul class={classes!("steps")}>
        {for props.children.iter()}
        </ul>
    }
}


#[derive(Properties,PartialEq)]
pub struct StepProps {
    pub children : Children,
    #[prop_or_default]
    pub step_classes :  &'static str
}

#[function_component(Step)]
pub fn step (props:&StepProps) -> Html {
    html!{
        <li class={classes!("step",props.step_classes)}>
        {for props.children.iter()}
        </li>
    }
}



