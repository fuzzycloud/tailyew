use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TableProps {
    pub children: Children,
    #[prop_or_default]
    pub table_classes: &'static str,
}

#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    html! {
        <div class={classes!("table")}>
            {for props.children.iter()}
        </div>
    }
}
