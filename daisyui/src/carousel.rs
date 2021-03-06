use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    html! {
        <div class={classes!("carousel",props.class.clone())}>
            {for props.children.iter()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CarouselItemProps {
    pub children: Children,
}

#[function_component(CarouselItem)]
pub fn carousel_item(props: &CarouselItemProps) -> Html {
    html! {
        <div class={classes!("carousel-item")}>
            {for props.children.iter()}
        </div>
    }
}
