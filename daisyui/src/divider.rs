use yew::prelude::*;

#[function_component(Divider)]
pub fn divider() -> Html {
    html! {
        <div class={classes!("divider")}></div>
    }
}
