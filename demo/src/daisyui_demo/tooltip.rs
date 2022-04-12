use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
       <Tooltip>
        <Button text="Button Example" />
       </Tooltip>
    }
}
