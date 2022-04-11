use yew::prelude::*;
use yew_router::prelude::*;
use crate::DaisyRoute;
use crate::daisyui_demo::{button, drawer,card,avatar,carousel,collapse};


fn switch(route: &DaisyRoute) -> Html {
    match route {
         DaisyRoute::Button => html! {<button::Demo />},
         DaisyRoute::Drawer => html! {<drawer::Demo />},
         DaisyRoute::Card => html! {<card::Demo />},        
         DaisyRoute::Avatar => html! {<avatar::Demo />},
         DaisyRoute::Carousel => html! {<carousel::Demo />},        
         DaisyRoute::Collapse => html! {<collapse::Demo />}        
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