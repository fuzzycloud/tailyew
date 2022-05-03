use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum ButtonBrandColors {
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,
}

#[derive(PartialEq, Clone)]
pub enum ButtonOutline {
    OutlinePrimary,
    OutlineSecondary,
    OutlineAccent,
    Outline,
}

#[derive(PartialEq, Clone)]
pub enum ButtonOutlineColor {
    OutlineColorInfo,
    OutlineColorSuccess,
    OutlineColorWarning,
    OutlineColorError,
}

#[derive(PartialEq, Clone)]
pub enum ButtonSize {
    Large,
    Small,
    ExtraSmall,
}

#[derive(PartialEq, Clone)]
pub enum ButtonTag {
    Link,
    Button,
    Input,
    Submit,
    Reset,
}

#[derive(PartialEq, Clone)]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

impl ButtonBrandColors {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonBrandColors::Primary => "btn-primary",
            ButtonBrandColors::Secondary => "btn-secondary",
            ButtonBrandColors::Accent => "btn-accent",
            ButtonBrandColors::Ghost => "btn-ghost",
            ButtonBrandColors::Link => "btn-link",
        }
    }
}

impl ButtonOutlineColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonOutlineColor::OutlineColorInfo => "btn-outline btn-info",
            ButtonOutlineColor::OutlineColorSuccess => "btn-outline btn-success",
            ButtonOutlineColor::OutlineColorWarning => "btn-outline btn-warning",
            ButtonOutlineColor::OutlineColorError => "btn-outline btn-error",
        }
    }
}

impl ButtonOutline {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonOutline::Outline => "btn-outline",
            ButtonOutline::OutlinePrimary => "btn-outline btn-primary",
            ButtonOutline::OutlineSecondary => "btn-outline btn-secondary",
            ButtonOutline::OutlineAccent => "btn-outline btn-accent",
        }
    }
}

impl ButtonSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonSize::Large => "btn-lg",
            ButtonSize::Small => "btn-sm",
            ButtonSize::ExtraSmall => "btn-xs",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub text: &'static str,
    #[prop_or_default]
    pub color: Option<ButtonBrandColors>,
    #[prop_or_default]
    pub outline: Option<ButtonOutline>,
    #[prop_or_default]
    pub outlinecolor: Option<ButtonOutlineColor>,
    #[prop_or_default]
    pub is_outline: bool,
    #[prop_or_default]
    pub size: Option<ButtonSize>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button
            onclick={props.onclick.clone()}
            class={classes!("btn",
            props.color.clone().map(|x| x.as_str()).unwrap_or_default(),
            props.size.clone().map( |x| x.as_str()).unwrap_or_default(),
            props.outline.clone().map( |x| x.as_str()).unwrap_or_default(),
            props.outlinecolor.clone().map( |x| x.as_str()).unwrap_or_default(),
            props.is_outline.then(|| "btn-outline"),
            props.class.clone()
        )}>{props.text}</button>
    }
}
