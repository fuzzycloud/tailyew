use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let indicator = html! {
      <Indicator>
        <IndicatorItem class="badge badge-secondary  pt-24">{"New"}
            <div class="grid w-32 h-32 bg-base-300 place-items-center ml-48 mt-24 ">{"content"}</div>
        </IndicatorItem>
      </Indicator>

    };
    html! {
        <div>
            <Display title="Indicator" preview={indicator} />
        </div>
    }
}
