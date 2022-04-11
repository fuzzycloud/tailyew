use yew::prelude::*;
use daisyui::prelude::*;


#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
        <Select select_classes="select-bordered w-48  max-w-xs">
            <option>{"Who shot first?"}</option>
            <option>{"Han Solo"}</option>
            <option>{"Greedo"}</option>
        </Select>
    }
}