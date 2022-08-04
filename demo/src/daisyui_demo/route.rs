use crate::DaisyRoute;
use demo_components::daisyui::{
    alert, avatar, badge, button, card, carousel, collapse, divider, drawer, dropdown, home,
    indicator, link, menu, modal, nav, radio, select, step, tab, table, text_input, textarea,
    toggle, tooltip,
};
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
        DaisyRoute::TextInput => html! {<text_input::Demo />},
        DaisyRoute::Select => html! {<select::Demo />},
        DaisyRoute::Toggle => html! {<toggle::Demo />},
        DaisyRoute::Indicator => html! {<indicator::Demo />},
        DaisyRoute::Menu => html! {<menu::Demo />},
        DaisyRoute::Step => html! {<step::Demo />},
        DaisyRoute::Tab => html! {<tab::Demo />},
        DaisyRoute::Link => html! {<link::Demo />},
        DaisyRoute::Nav => html! {<nav::Demo />},
        DaisyRoute::Home => html! {<home::Demo />},
        DaisyRoute::Modal => html! {<modal::Demo />},
        DaisyRoute::Dropdown => html! {<dropdown::Demo />},
        DaisyRoute::Badge => html! {<badge::Demo />},
        DaisyRoute::Textarea => html! {<textarea::Demo />},
        DaisyRoute::Radio => html! {<radio::Demo />},
        DaisyRoute::Divider => html! {<divider::Demo />},
        DaisyRoute::Alert => html! {<alert::Demo />},
    }
}

#[derive(Clone, PartialEq)]
struct DemoRoute {
    pub title: &'static str,
    pub route: DaisyRoute,
}

#[function_component(Route)]
pub fn route() -> Html {
    let routes = vec![
        DemoRoute {
            title: "Button",
            route: DaisyRoute::Button,
        },
        DemoRoute {
            title: "Drawer",
            route: DaisyRoute::Drawer,
        },
        DemoRoute {
            title: "Card",
            route: DaisyRoute::Card,
        },
        DemoRoute {
            title: "Avatar",
            route: DaisyRoute::Avatar,
        },
        DemoRoute {
            title: "Carousel",
            route: DaisyRoute::Carousel,
        },
        DemoRoute {
            title: "Collapse",
            route: DaisyRoute::Collapse,
        },
        DemoRoute {
            title: "Table",
            route: DaisyRoute::Table,
        },
        DemoRoute {
            title: "Tooltip",
            route: DaisyRoute::Tooltip,
        },
        DemoRoute {
            title: "TextInput",
            route: DaisyRoute::TextInput,
        },
        DemoRoute {
            title: "Select",
            route: DaisyRoute::Select,
        },
        DemoRoute {
            title: "Toggle",
            route: DaisyRoute::Toggle,
        },
        DemoRoute {
            title: "Indicator",
            route: DaisyRoute::Indicator,
        },
        DemoRoute {
            title: "Menu",
            route: DaisyRoute::Menu,
        },
        DemoRoute {
            title: "Step",
            route: DaisyRoute::Step,
        },
        DemoRoute {
            title: "Tab",
            route: DaisyRoute::Tab,
        },
        DemoRoute {
            title: "Link",
            route: DaisyRoute::Link,
        },
        DemoRoute {
            title: "Alert",
            route: DaisyRoute::Alert,
        },
        DemoRoute {
            title: "Nav",
            route: DaisyRoute::Nav,
        },
        DemoRoute {
            title: "Home",
            route: DaisyRoute::Home,
        },
        DemoRoute {
            title: "Modal",
            route: DaisyRoute::Modal,
        },
        DemoRoute {
            title: "Dropdown",
            route: DaisyRoute::Dropdown,
        },
        DemoRoute {
            title: "Badge",
            route: DaisyRoute::Badge,
        },
        DemoRoute {
            title: "Textarea",
            route: DaisyRoute::Textarea,
        },
        DemoRoute {
            title: "Radio",
            route: DaisyRoute::Radio,
        },
        DemoRoute {
            title: "Divider",
            route: DaisyRoute::Divider,
        },
    ];

    html! {
        <div>
            <div class="flex">
            <div class="w-1/5 ">
                <div class="drawer-mobile">
                    <input id="my-drawer" type="checkbox" class="drawer-toggle"/>
                        <div class="flex flex-col items-center justify-center">
                            <label for="my-drawer" class="btn btn-primary drawer-button lg:hidden">{"Open drawer"}</label>
                        </div>
                        <div>
                            <label for="my-drawer" class="drawer-overlay"></label>
                                <div class="drawer-side  ">
                                    <ul class="menu p-4 bg-base-200 overflow-y-auto w-80 bg-base-100 text-base-content">
                                        {routes.iter().map(|route| {
                                            html! {
                                                <li>
                                                    <Link<DaisyRoute> to={route.route.clone()}>
                                                         {route.title}
                                                    </Link<DaisyRoute>>
                                                </li>
                                            }
                                        }).collect::<Html>()}
                                    </ul>
                                </div>
                            </div>
                        </div>
                     </div>
                        <div class="w-4/5">
                        <Switch<DaisyRoute> render={Switch::render(switch)} />
                        </div>
            </div>
        </div>
    }
}
