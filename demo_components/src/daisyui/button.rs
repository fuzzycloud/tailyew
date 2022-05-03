use crate::daisyui::display_helper::*;
use daisyui::prelude::*;
use yew::prelude::*;

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
    let button_with_outline = html! {
        <>
            <Button text="Primary" outline={ButtonOutline::OutlinePrimary}  />
            <Button text="Secondary" outline={ButtonOutline::OutlineSecondary}  />
            <Button text="Accent" outline={ButtonOutline::OutlineAccent}  />
        </>
    };

    let button_with_outline_color = html! {
        <>
            <Button text="Info" outlinecolor={ButtonOutlineColor::OutlineColorInfo}  />
            <Button text="Success" outlinecolor={ButtonOutlineColor::OutlineColorSuccess}  />
            <Button text="Warning" outlinecolor={ButtonOutlineColor::OutlineColorWarning}  />
            <Button text="Error" outlinecolor={ButtonOutlineColor::OutlineColorError}  />

        </>
    };

    html! {
        <div>
            <Display title="button" preview={button} />
            <Display title="Buttons with brand colors" preview={button_with_brand} />
            <Display title="OutLine Button" preview={button_with_outline} />
            <Display title="OutLine Button With Message" preview={button_with_outline_color} />

        </div>
    }
}
