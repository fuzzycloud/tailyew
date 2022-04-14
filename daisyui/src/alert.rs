use yew::prelude::*;

// #[derive(Properties, PartialEq)]
// pub struct AlertProps {
//     pub children: Children,
//     #[prop_or_default]
//     pub alert_classes: &'static str,
// }

// #[function_component(Alert)]
// pub fn alert(props: &AlertProps) -> Html {
//     html! {
//         <div class={classes!("alert", props.alert_classes)}>
//         {for props.children.iter()}
//         </div>
//     }
// }

#[derive(PartialEq, Clone)]

pub enum AlertType {
    Alert,
    AlertInfo,
    AlertSuccess,
    AlertWarning,
    AlertError,
}

impl AlertType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertType::Alert => "alert",
            AlertType::AlertInfo => "alert-info",
            AlertType::AlertSuccess => "alert-success",
            AlertType::AlertWarning => "alert-warning",
            AlertType::AlertError => "alert-error",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    #[prop_or_default]
    pub text: &'static str,
    #[prop_or_default]
    pub color: Option<AlertType>,
}

#[function_component(Alert)]
pub fn allert(props: &AlertProps) -> Html {
    html! {
        <div class={classes!(
            "alert",
            props.color.clone().map(|x| x.as_str()).unwrap_or_default(),

        )}>{props.text}</div>
    }
}
