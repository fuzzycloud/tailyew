use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
        <div>
            <Drawer>
            <input id="my-drawer" type="checkbox" class="drawer-toggle"/>
            <DrawerContent>
            <label for="my-drawer" class="btn btn-primary drawer-button">{"Open drawer"}</label>
            </DrawerContent>
            <DrawerSide>
                <label for="my-drawer" class="drawer-overlay"></label>
                <ul class="menu p-4 overflow-y-auto w-80 bg-base-100 text-base-content">
                      <li><a>{"Sidebar Item 1"}</a></li>
                      <li><a>{"Sidebar Item 2"}</a></li>
                    </ul>
            </DrawerSide>
            </Drawer>
        </div>
    }
}
