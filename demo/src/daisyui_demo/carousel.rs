use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let carousel = html! {

        <Carousel class=" rounded-box">
            <CarouselItem >
                <img src="https://api.lorem.space/image/burger?w=400&h=300&hash=8B7BCDC2" alt="Burger" />
            </CarouselItem>
            <CarouselItem>
               <img src="https://api.lorem.space/image/burger?w=400&h=300&hash=500B67FB" alt="Burger" />
            </CarouselItem>
       </Carousel>
    };

    html! {
        <div>
            <Display title="Carousel" preview={carousel} />
        </div>
    }
}
