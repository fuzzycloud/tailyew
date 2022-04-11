use yew::prelude::*;
use yew_router::prelude::*;
use crate::DaisyRoute;
use crate::daisyui_demo::{button, drawer,card,avatar,carousel,collapse,table,tooltip,input,select,
toggle,indicator,menu,step,tab,link};


fn switch(route: &DaisyRoute) -> Html {
    match route {
         DaisyRoute::Button => html! {<button::Demo />},
         DaisyRoute::Drawer => html! {<drawer::Demo />},
         DaisyRoute::Card => html! {<card::Demo />},        
         DaisyRoute::Avatar => html! {<avatar::Demo />},
         DaisyRoute::Carousel => html! {<carousel::Demo />},        
         DaisyRoute::Collapse => html! {<collapse::Demo />},
         DaisyRoute::Table => html! {<table::Demo />},       
         DaisyRoute::Tooltip => html! {<tooltip::Demo />},  
         DaisyRoute::Input => html! {<input::Demo />},       
         DaisyRoute::Select => html! {<select::Demo />},             
         DaisyRoute::Toggle => html! {<toggle::Demo />},
         DaisyRoute::Indicator => html! {<indicator::Demo />},       
         DaisyRoute::Menu => html! {<menu::Demo />},       
         DaisyRoute::Step => html! {<step::Demo />},       
         DaisyRoute::Tab => html! {<tab::Demo />},       
         DaisyRoute::Link => html! {<link::Demo />},       

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