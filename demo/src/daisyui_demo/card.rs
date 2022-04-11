use yew::prelude::*;
use daisyui::prelude::*;

// <div class="card w-96 bg-base-100 shadow-xl">
//   <figure><img src="https://api.lorem.space/image/shoes?w=400&h=225" alt="Shoes" /></figure>
//   <div class="card-body">
//     <h2 class="card-title">Shoes!</h2>
//     <p>If a dog chews shoes whose shoes does he choose?</p>
//     <div class="card-actions justify-end">
//       <button class="btn btn-primary">Buy Now</button>
//     </div>
//   </div>
// </div>

#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
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
    }
}