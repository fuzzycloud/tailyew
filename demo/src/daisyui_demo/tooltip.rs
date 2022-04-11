use yew::prelude::*;
use daisyui::prelude::*;


#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
       <Tooltip>
       {"Hello"}
        <Button text="Button Example" />
       </Tooltip>
    }
}