use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct CollapseProps {
    pub children : Children
}

#[function_component(Collapse)]
pub fn collapse (props:&CollapseProps) -> Html {
    html!{
        <div class={classes!("collapse")}>
            {for props.children.iter()}
        </div>
    }
}


#[derive(Properties,PartialEq)]
pub struct CollapseTitleProps {
    pub children : Children
}


#[function_component(CollapseTitle)]
pub fn collapse_title (props:&CollapseTitleProps) -> Html {
    html!{
        <div class={classes!("collapse-title")}>
            {for props.children.iter()}
        </div>
    }
}


#[derive(Properties,PartialEq)]
pub struct CollapseContentProps {
    pub children : Children
}


#[function_component(CollapseContent)]
pub fn collapse_content (props:&CollapseContentProps) -> Html {
    html!{
        <div class={classes!("collapse-content")}>
            {for props.children.iter()}
        </div>
    }
}