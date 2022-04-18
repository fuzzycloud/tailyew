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
    let card = html! {

        <Card card_classes="w-96 bg-base-100 shadow-xl">
            <figure>
                <img src="https://api.lorem.space/image/shoes?w=400&h=225" alt="Shoes" />
            </figure>
            <CardBody>
                <CardTitle> {"Shoes!"} </CardTitle>
                <p>{"If a dog chews shoes whose shoes does he choose?"}</p>
            <CardActions>
                <div class="card-actions justify-end">
                    <Button text="Buy Now" color={ButtonBrandColors::Primary} />
                </div>
            </CardActions>

            </CardBody>
        </Card>
    };

    html! {
        <div>
            <Display title="Card" preview={card} />
        </div>
    }
}
