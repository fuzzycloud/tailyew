use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <div>
            <h1 class={classes!("text-3xl", "font-bold", "underline")}>
                {"Demo and Documentation coming soon..."}
            </h1>

            </div>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
