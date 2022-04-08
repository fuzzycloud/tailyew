use yew::prelude::*;
use yew_router::prelude::*;

mod home;
mod daisyui;

#[derive(Clone, Routable, PartialEq)]
pub enum DaisyRoute {
    #[at("/daisyUI/button")]
    Button,
    #[at("/daisyUI/drawer")]
    Drawer
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/daisyUI")]
    DaisyUIRoot,
    #[at("/daisyUI/*")]
    DaisyUI
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <home::Route />
        },
        Route::DaisyUI | Route::DaisyUIRoot => html! {
            <daisyui::route::Route />
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
