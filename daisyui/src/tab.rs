use yew::prelude::*;

#[derive(Properties,PartialEq)]
pub struct TabsProps {
    pub children : Children,
}

#[function_component(Tabs)]
pub fn tabs (props:&TabsProps) -> Html {
    html!{
        <div class={classes!("tabs")}>
        {for props.children.iter()}
        </div>
    }
}


#[derive(Properties,PartialEq)]
pub struct TabProps {
    pub children : Children,
    #[prop_or_default]
    pub tab_classes :  &'static str
}

#[function_component(Tab)]
pub fn tab (props:&TabProps) -> Html {
    html!{
        <li class={classes!("tab",props.tab_classes)}>
        {for props.children.iter()}
        </li>
    }
}



