use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
       <Avatar>
       <div class="w-24 rounded">
            <img src="https://api.lorem.space/image/face?hash=92048" />
       </div>
       </Avatar>
    }
}
