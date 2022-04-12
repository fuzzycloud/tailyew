use crate::daisyui_demo::{button, drawer};
use crate::DaisyRoute;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: &DaisyRoute) -> Html {
    match route {
        DaisyRoute::Button => html! {<button::Demo />},
        DaisyRoute::Drawer => html! {<drawer::Demo />},
    }
}

#[function_component(Route)]
pub fn route() -> Html {
    html! {
        <div>
            <div>
            {"Drawer menu will come here"}

            </div>
            <div>
                <Switch<DaisyRoute> render={Switch::render(switch)} />
            </div>
        </div>
    }
}
