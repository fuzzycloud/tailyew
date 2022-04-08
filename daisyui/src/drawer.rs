use yew::prelude::*; 

#[derive(Properties, PartialEq)]
pub struct DrawerProps {
    pub children : Children
}

#[function_component(Drawer)]
pub fn drawer (props: &DrawerProps) -> Html {
    // let test = vec!["drawer-mobile"];
    html! {
        <div class={classes!("drawer")}> 
        { for props.children.iter() }
        </div>
         
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerContentProps {
    pub children: Children
}

#[function_component(DrawerContent)]
pub fn drawer_content(props : &DrawerContentProps) -> Html{
    html! {
        <div class = {classes!("drawer-content")}> 
        { for props.children.iter() }
        </div>
         
    }
}

#[derive(Properties, PartialEq)]
pub struct DrawerSliderProps {
    pub children : Children
}

#[function_component(DrawerSlider)]
pub fn drawer_slider(props : &DrawerSliderProps) -> Html {
    html! {
        <div class = {classes!("drawer-slider")}> 
        { for props.children.iter() }
        </div>
         
    }
}