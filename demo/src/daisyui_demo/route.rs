use crate::daisyui_demo::{
    alert, avatar, button, card, carousel, collapse, drawer, home, indicator, input, link, menu,
    nav, select, step, tab, table, toggle, tooltip,
};
use crate::DaisyRoute;
use yew::prelude::*;
use yew_router::prelude::*;

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
        DaisyRoute::Alert => html! {<alert::Demo />},
        DaisyRoute::Nav => html! {<nav::Demo />},
        DaisyRoute::Home => html! {<home::Demo />},
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
