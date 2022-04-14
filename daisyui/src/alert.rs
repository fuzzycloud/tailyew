use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum AlertType {
    Info,
    Success,
    Warning,
    Error,
}

impl AlertType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertType::Info => "alert-info",
            AlertType::Success => "alert-success",
            AlertType::Warning => "alert-warning",
            AlertType::Error => "alert-error",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub color: Option<AlertType>,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
        <div class={classes!(
            "alert",
            props.color.clone().map(|x| x.as_str()).unwrap_or_default(),
        )}>
            {for props.children.iter()}
        </div>
    }
}
