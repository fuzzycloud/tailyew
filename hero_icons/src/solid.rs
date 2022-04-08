use yew::prelude::*;
use crate::icons::*;

#[function_component(AdjustmentsIconSolid)]
pub fn adjustments_solid () -> Html {
    html! {
        <AdjustmentsIcon icon_type={IconType::Solid} />

    }
}

#[function_component(ArrowLeftIconSolid)]
pub fn arrow_left_solid () -> Html{
    html!{
        <ArrowLeftIcon icon_type={IconType::Solid} />

    }
}