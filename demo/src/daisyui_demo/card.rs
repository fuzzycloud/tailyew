use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let card = html! {

        <Card class="w-96 bg-base-100 shadow-xl">
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
