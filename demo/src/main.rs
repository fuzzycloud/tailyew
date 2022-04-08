use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod daisyui_demo;
mod hero_icons_demo;

#[derive(Clone, Routable, PartialEq)]
pub enum DaisyRoute {
    #[at("/daisyui/button")]
    Button,
    #[at("/daisyui/drawer")]
    Drawer
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/daisyui")]
    DaisyUIRoot,
    #[at("/daisyui/*")]
    DaisyUI,
    #[at("/heroicons")]
    HeroIconsRoot,
    #[at("/heroicons/*")]
    HeroIcons,

}

#[derive(Clone, Routable, PartialEq)]
pub enum HeroIconsRoute {
    #[at("/heroicons/outlined")]
    OutLined,
    #[at("/heroicons/solid")]
    Solid,

}


fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <home::Route />
        },
        Route::DaisyUI | Route::DaisyUIRoot => html! {
            <daisyui_demo::route::Route />
        },
        Route::HeroIcons | Route::HeroIconsRoot => html! {
            <hero_icons_demo::route::Route />
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
