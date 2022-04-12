use crate::hero_icons_demo::{outlined, solid};
use crate::HeroIconsRoute;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(route: &HeroIconsRoute) -> Html {
    match route {
        HeroIconsRoute::OutLined => html! {<outlined::Demos />},
        HeroIconsRoute::Solid => html! {<solid::Demo />},
    }
}

#[function_component(Route)]
pub fn route() -> Html {
    html! {
        <div>
            <div>
            {"Icons will come here"}

            </div>
            <div>
                <Switch<HeroIconsRoute> render={Switch::render(switch)} />
            </div>
        </div>
    }
}
