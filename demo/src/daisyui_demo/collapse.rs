use daisyui::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DisplayProp {
    title: &'static str,
    // code : &'static str,
    preview: Html,
}

#[function_component(Display)]
pub fn display(props: &DisplayProp) -> Html {
    html! {
        <div class="m-12">
           <Card card_classes="bg-base-100 shadow-xl">
                <CardBody>
                    <CardTitle>
                        <p> {props.title} </p>
                    </CardTitle>
                    {props.preview.clone()}
                </CardBody>
            </Card>
        </div>
    }
}

#[function_component(Demo)]
pub fn demo() -> Html {
    let collapse = html! {

      <Collapse>
          <CollapseTitle collapse_title_classes="text-xl font-medium">{" Focus me to see content"}</CollapseTitle>
          <CollapseContent>{"attribute is necessary to make the div focusable"}</CollapseContent>
      </Collapse>
    };

    html! {
        <div>
            <Display title="Collapse" preview={collapse} />
        </div>
    }
}
