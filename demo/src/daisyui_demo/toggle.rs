use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let toggle = html! {
           <div class="form-control">
                <Label label_classes="cursor-pointer">
                  <LabelText>
                  {"Remember me"}
                  </LabelText>
                </Label>
           </div>

    };

    html! {
        <div>
            <Display title="Toggle" preview={toggle} />
        </div>
    }
}
