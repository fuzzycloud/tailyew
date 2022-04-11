use yew::prelude::*;
use daisyui::prelude::*;


#[function_component(Demo)]
pub fn demo() -> Html {
    html! {
           <div class="form-control">
                <Label label_classes="cursor-pointer">
                  <LabelText>
                  {"Remember me"}
                  </LabelText>
                //   <Toggle>
                    // <Input input_classes="toggle-primary" input_type={InputTypes::Checkbox}/>
                //   </Toggle>
                </Label>
           </div>
            
    }
}