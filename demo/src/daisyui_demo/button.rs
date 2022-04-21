use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DisplayProp {
    title: &'static str,
    preview: Html,
}

#[function_component(Demo)]
pub fn demo() -> Html {
    let button = html! {

        <Button text="Sample Button" />
    };

    let button_with_brand = html! {
        <>
            <Button text="Button"  />
            <Button text="Primary" color={ButtonBrandColors::Primary}  />
            <Button text="Secondary" color={ButtonBrandColors::Secondary}  />
            <Button text="Link" color={ButtonBrandColors::Link}  />
            <Button text="Ghost" color={ButtonBrandColors::Ghost}  />
            <Button text="Accent" color={ButtonBrandColors::Accent}  />
        </>
    };

    html! {
        <div>
            <Display title="button" preview={button} />
            <Display title="Buttons with brand colors" preview={button_with_brand} />
        </div>
    }
}
