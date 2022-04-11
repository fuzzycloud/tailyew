use yew::prelude::*;
use daisyui::prelude::*;

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
      <Collapse>
            <CollapseTitle collapse_title_classes="text-xl font-medium">{" Focus me to see content"}</CollapseTitle>
            <CollapseContent>{"attribute is necessary to make the div focusable"}</CollapseContent>

      </Collapse>

    }
}