use yew::prelude::*;
use daisyui::prelude::*;

// <div class="avatar">
//   <div class="w-24 rounded">
//     <img src="https://api.lorem.space/image/face?hash=92048" />
//   </div>
// </div>

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