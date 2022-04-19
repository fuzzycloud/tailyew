use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let collapse = html! {

      <Collapse>
          <CollapseTitle collapse_title_classes="text-xl font-medium">{" Focus me to see content"}</CollapseTitle>
          <CollapseContent>{"attribute is necessary to make the div focusable"}</CollapseContent>
      </Collapse>
    };

    html! {
        <div>
            <Display title="Collapse" preview={collapse} />
        </div>
    }
}
