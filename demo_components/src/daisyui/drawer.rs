use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let drawer = html! {
        <div>
            <Drawer class="drawer-mobile">
            <input id="my-drawer" type="checkbox" class="drawer-toggle"/>
            <DrawerContent class="flex flex-col items-center justify-center">
            <label for="my-drawer" class="btn btn-primary drawer-button lg:hidden">{"Open drawer"}</label>
            </DrawerContent>
            <DrawerSide class="">
                <label for="my-drawer" class="drawer-overlay"></label>
                <ul class="menu p-4 overflow-y-auto w-80 bg-base-100 text-base-content">
                      <li><a>{"Sidebar Item 1"}</a></li>
                      <li><a>{"Sidebar Item 2"}</a></li>
                    </ul>
            </DrawerSide>
            </Drawer>
        </div>
    };

    html! {
        <div>
            <Display title="Drawer" preview={drawer} />
        </div>
    }
}
