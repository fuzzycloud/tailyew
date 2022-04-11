use yew::prelude::*;
use daisyui::prelude::*;



#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
       <Carousel carousel_classes=" rounded-box">
            <CarouselItem >
                <img src="https://api.lorem.space/image/burger?w=400&h=300&hash=8B7BCDC2" alt="Burger" />               
            </CarouselItem>
            <CarouselItem>
               <img src="https://api.lorem.space/image/burger?w=400&h=300&hash=500B67FB" alt="Burger" />
            </CarouselItem>
       </Carousel>

    }
}