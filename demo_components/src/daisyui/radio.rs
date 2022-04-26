use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let radio = html! {
        <div class="form-control">
        <Label class="cursor-pointer">
            <Label class="text-3xl">{"Red Pil"}
                <input type="radio" name="radio-6" class="radio checked:bg-red-500" />
            </Label>
        </Label>
        </div>

    };

    html! {
        <div>
            <Display title="Radio" preview={radio} />
        </div>
    }
}
