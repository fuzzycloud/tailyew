use daisyui::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DisplayProp {
    title: &'static str,
    // code : &'static str,
    preview: Html,
}

#[function_component(Display)]
pub fn display(props: &DisplayProp) -> Html {
    html! {
        <div class="m-12">
           <Card card_classes="bg-base-100 shadow-xl">
                <CardBody>
                    <CardTitle>
                        <p> {props.title} </p>
                    </CardTitle>
                    {props.preview.clone()}
                </CardBody>
            </Card>
        </div>
    }
}

#[function_component(Demo)]
pub fn demo() -> Html {
    let carousel = html! {

        <Carousel carousel_classes=" rounded-box">
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
