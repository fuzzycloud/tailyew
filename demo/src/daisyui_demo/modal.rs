use crate::daisyui_demo::display_prop::Display;
use daisyui::prelude::*;
use yew::prelude::*;

#[function_component(Demo)]
pub fn demo() -> Html {
    let modal = html! {
        <>
        <label class="btn modal-button" for="my-modal">{"Open Modal"}</label>
            <input type="checkbox" id="my-modal" class="modal-toggle" />
            <Modal>
                <ModalBox>
                        <h3 class="font-bold text-lg">{"Congratulations random Interner user!"}</h3>
                        <p class="py-4">{"You've been selected for a chance to get one year of subscription to use Wikipedia for free!"}</p>
                        <ModalAction>
                            <label for="my-modal" class="btn">{"Yay!"}</label>
                        </ModalAction>
                </ModalBox>
            </Modal>
            </>


    };
    html! {
        <div>
            <Display title="Modal" preview={modal} />
        </div>
    }
}
