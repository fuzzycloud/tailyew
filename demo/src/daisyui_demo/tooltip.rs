use yew::prelude::*;
use daisyui::prelude::*;


#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
       <Tooltip>
        <Button text="Button Example" />
       </Tooltip>
    }
}