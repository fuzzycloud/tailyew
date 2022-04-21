use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
        <Drawer class="drawer-mobile">
            <input id="my-drawer" type="checkbox" class="drawer-toggle"/>
            <DrawerContent class="flex flex-col items-center justify-center">
            <label for="my-drawer" class="btn btn-primary drawer-button lg:hidden">{"Open drawer"}</label>
            </DrawerContent>
            <DrawerSide class="">
                <label for="my-drawer" class="drawer-overlay"></label>
                <ul class="menu p-4 overflow-y-auto w-80 bg-base-100 text-base-content">
                    <li><a>{"Button"}</a></li>
                    <li><a>{"Dropdown"}</a></li>
                    <li><a>{"Modal"}</a></li>
                    <li><a>{"Swap"}</a></li>
                    <li><a>{"Alert"}</a></li>
                    <li><a>{"Avatar"}</a></li>
                    <li><a>{"Badge"}</a></li>
                    <li><a>{"Card"}</a></li>
                    <li><a>{"Carousel"}</a></li>
                    <li><a>{"Collapse"}</a></li>
                    <li><a>{"Countdown"}</a></li>
                    <li><a>{"Kbd"}</a></li>
                    <li><a>{"Progress"}</a></li>
                    <li><a>{"Radial Progress"}</a></li>
                    <li><a>{"Star"}</a></li>
                    <li><a>{"Table"}</a></li>
                    <li><a>{"Tooltip"}</a></li>
                    <li><a>{"Ceckbox"}</a></li>
                    <li><a>{"Text input"}</a></li>
                    <li><a>{"Radio"}</a></li>
                    <li><a>{"Range"}</a></li>
                    <li><a>{"Rating"}</a></li>
                    <li><a>{"Select"}</a></li>
                    <li><a>{"Textarea"}</a></li>
                    <li><a>{"Toggle"}</a></li>
                    <li><a>{"Artboard"}</a></li>
                    <li><a>{"Button group"}</a></li>
                    <li><a>{"Divider"}</a></li>
                    <li><a>{"Drawer"}</a></li>
                    <li><a>{"Footer"}</a></li>
                    <li><a>{"Hero"}</a></li>
                    <li><a>{"Indicator"}</a></li>
                    <li><a>{"Input group"}</a></li>
                    <li><a>{"Mask"}</a></li>
                    <li><a>{"Stack"}</a></li>
                    <li><a>{"Breadcrumbs"}</a></li>
                    <li><a>{"Link"}</a></li>
                    <li><a>{"Menu"}</a></li>
                    <li><a>{"Navbar"}</a></li>
                    <li><a>{"Pagination"}</a></li>
                    <li><a>{"Steps"}</a></li>
                    <li><a>{"Tab"}</a></li>
                    </ul>
            </DrawerSide>
        </Drawer>



    }
}
