use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let select = html! {
        <Select class="select-bordered w-48  max-w-xs">
            <option>{"Who shot first?"}</option>
            <option>{"Han Solo"}</option>
            <option>{"Greedo"}</option>
        </Select>
    };
    html! {
        <div>
            <Display title="Select" preview={select} />
        </div>
    }
}
