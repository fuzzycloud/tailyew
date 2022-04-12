use daisyui::prelude::*;
use yew::prelude::*;

// <div tabindex="0" class="collapse">
//   <div class="collapse-title text-xl font-medium">
//     Focus me to see content
//   </div>
//   <div class="collapse-content">
//     <p>tabindex="0" attribute is necessary to make the div focusable</p>
//   </div>
// </div>

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
      <Indicator>
        <IndicatorItem indicator_item_classes="badge badge-secondary  pt-24">{"New"}
            <div class="grid w-32 h-32 bg-base-300 place-items-center ml-48 mt-24 ">{"content"}</div>
        </IndicatorItem>
      </Indicator>

    }
}
