pub use crate::daisyui_demo::drawer::Demo;
use crate::daisyui_demo::{
    alert, avatar, badge, button, card, carousel, collapse, drawer, dropdown, home, indicator,
    input, link, menu, modal, nav, select, step, tab, table, textarea, toggle, tooltip,
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
        DaisyRoute::Modal => html! {<modal::Demo />},
        DaisyRoute::Dropdown => html! {<dropdown::Demo />},
        DaisyRoute::Badge => html! {<badge::Demo />},
        DaisyRoute::Textarea => html! {<textarea::Demo />},
    }
}

#[function_component(Route)]
pub fn route() -> Html {
    let alert = "Alert";
    let avatar = "Avatar";
    let button = "Button";
    let card = "Card";
    let carousel = "Carousel";
    let collapse = "Collapse";
    let drawer = "Drawer";
    let home = "Home";
    let indicator = "Indicator";
    let input = "Input";
    let link = "Link";
    let menu = "Menu";
    let modal = "Modal";
    let nav = "Nav";
    let select = "Select";
    let step = "Step";
    let tab = "Tab";
    let table = "Table";
    let toggle = "Toggle";
    let tooltip = "Tooltip";
    let modal = "Modal";
    let dropdown = "Dropdown";
    let badge = "Badge";
    let textarea = "Textarea";

    html! {
        <div>
            <div class="flex">
            <div class="w-1/5 ">
                <div class="drawer-mobile ">
                    <input id="my-drawer" type="checkbox" class="drawer-toggle"/>
                        <div class="flex flex-col items-center justify-center">
                            <label for="my-drawer" class="btn btn-primary drawer-button lg:hidden">{"Open drawer"}</label>
                        </div>
                        <div>
                            <label for="my-drawer" class="drawer-overlay"></label>
                                <div class="drawer-side  ">
                                    <ul class="menu p-4 bg-base-200 overflow-y-auto w-80 bg-base-100 text-base-content">
                                        <li>

                                           <Link<DaisyRoute> to={DaisyRoute::Button}>
                                            {"Button"}
                                            </Link<DaisyRoute>>

                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Dropdown}>
                                            {"Dropdown"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Modal}>
                                            {"Modal"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Alert}>
                                            {"Alert"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Avatar}>
                                            {"Avatar"}
                                            </Link<DaisyRoute>>
                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Card}>
                                            {"Card"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Carousel}>
                                            {"Carousel"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Collapse}>
                                            {"Collapse"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Table}>
                                            {"Table"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Tooltip}>
                                            {"Tooltip"}
                                            </Link<DaisyRoute>>
                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Select}>
                                            {"Select"}
                                            </Link<DaisyRoute>>
                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Toggle}>
                                            {"Toggle"}
                                            </Link<DaisyRoute>>
                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Drawer}>
                                            {"Drawer"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Indicator}>
                                            {"Indicator"}
                                            </Link<DaisyRoute>>
                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Step}>
                                            {"Step"}
                                            </Link<DaisyRoute>>
                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Link}>
                                            {"Link"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Menu}>
                                            {"Menu"}
                                            </Link<DaisyRoute>>
                                        </li>

                                        <li>
                                            <Link<DaisyRoute> to={DaisyRoute::Tab}>
                                            {"Tab"}
                                            </Link<DaisyRoute>>
                                        </li>
                                        <li>
                                        <Link<DaisyRoute> to={DaisyRoute::Badge}>
                                        {"Badge"}
                                        </Link<DaisyRoute>>
                                    </li>
                                    <li>
                                    <Link<DaisyRoute> to={DaisyRoute::Textarea}>
                                    {"Textarea"}
                                    </Link<DaisyRoute>>
                                </li>
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

//  <li>
// <Link<DaisyRoute> to={DaisyRoute::Breadcrumbs}>
// {"Breadcrumbs"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Inputgroup}>
// {"Input group"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Buttongroup}>
// {"Buttongroup"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Divider}>
// {"Divider"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Textarea}>
// {"Textarea"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Ceckbox}>
// {"Ceckbox"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Textinput}>
// {"Textinput"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Radio}>
// {"Radio"}
// </Link<DaisyRoute>>
// </li>
// <li>
//     <Link<DaisyRoute> to={DaisyRoute::Badge}>
//     {"Badge"}
//     </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Navbar}>
// {"Navbar"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Pagination}>
// {"Pagination"}
// </Link<DaisyRoute>>
// </li>
// <li>
// <Link<DaisyRoute> to={DaisyRoute::Steps}>
// {"Steps"}
// </Link<DaisyRoute>>
// </li>
