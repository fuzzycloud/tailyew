use yew::prelude::*;
use yew_router::prelude::*;

mod daisyui_demo;
mod hero_icons_demo;
mod home;

#[derive(Clone, Routable, PartialEq)]
pub enum DaisyRoute {
    #[at("/daisyui/button")]
    Button,
    #[at("/daisyui/drawer")]
    Drawer,
    #[at("/daisyui/card")]
    Card,
    #[at("/daisyui/avatar")]
    Avatar,
    #[at("/daisyui/carousel")]
    Carousel,
    #[at("/daisyui/collapse")]
    Collapse,
    #[at("/daisyui/table")]
    Table,
    #[at("/daisyui/tootip")]
    Tooltip,
    #[at("/daisyui/text_input")]
    TextInput,
    #[at("/daisyui/select")]
    Select,
    #[at("/daisyui/toggle")]
    Toggle,
    #[at("/daisyui/indicator")]
    Indicator,
    #[at("/daisyui/menu")]
    Menu,
    #[at("/daisyui/step")]
    Step,
    #[at("/daisyui/tab")]
    Tab,
    #[at("/daisyui/link")]
    Link,
    // #[at("/daisyui/alert")]
    // Alert,
    #[at("/daisyui/nav")]
    Nav,
    #[at("/daisyui/home")]
    Home,
    #[at("/daisyui/modal")]
    Modal,
    #[at("/daisyui/dropdown")]
    Dropdown,
    #[at("/daisyui/badge")]
    Badge,
    #[at("/daisyui/textarea")]
    Textarea,
    #[at("/daisyui/radio")]
    Radio,
    #[at("/daisyui/divider")]
    Divider,
    #[at("/daisyui/alert")]
    Alert,
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
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={|route: Route| switch(&route)} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
