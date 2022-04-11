use yew::prelude::*;
use hero_icons::prelude::*;

#[function_component(Demos)]
pub fn demos() -> Html {
    html! {
        <>
        <div class="grid grid-cols-9 gap-4">
                <AdjustmentsIconOutLined />
                <ArrowNarrowUpOutLined/>
                <ArrowLeftIconOutLined/>
                <ArrowNarrowDownOutlined/>
                <CheckOutLined/>
                <ChevronDownOutLined/>
                
        </div>
         </>


         
    }
}