use::yew::prelude::*;
use yew::{Properties};

#[derive(PartialEq, Clone)]
pub enum ButtonBrandColors {
    Primary,
    Secondary,
    Accent,
    Ghost,
    Link,
  }
#[derive(PartialEq, Clone)]
pub enum ButtonSize{
    Large,
    Small,
    ExtraSmall,
}

#[derive(PartialEq, Clone)]
pub enum ButtonTag{
    Link,
    Button,
    Input,
    Submit,
    Reset,
}

#[derive(PartialEq, Clone)]

pub enum ButtonType{
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

impl ButtonSize{
    pub fn as_str(&self) -> &'static str {
        match self{
            ButtonSize::Large => "btn-lg",
            ButtonSize::Small => "btn-sm",
            ButtonSize::ExtraSmall => "btn-xs"
        }
    }
}


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub text : &'static str,
    #[prop_or_default]
    pub color: Option<ButtonBrandColors>,
    #[prop_or_default]
    pub is_outline : bool,
    #[prop_or_default]
    pub size: Option<ButtonSize>
}

#[function_component(Button)]
pub fn button (props : &ButtonProps) -> Html {
    html! {
        <button class={classes!(
            "btn",
            props.color.clone().map(|x| x.as_str()).unwrap_or_default(),
            props.size.clone().map( |x| x.as_str()).unwrap_or_default(),
            props.is_outline.then(|| "btn-outline")
        )}>{props.text}</button>
    }
}


#[derive(Properties, PartialEq)]
pub struct ButtonControlProps {
    pub children : Children
}

