use gloo::{events::EventListener, utils::document};
use uml_common::{
    id::Id,
    prompt::{Prompt, PromptResponse},
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlDialogElement, HtmlFormElement, HtmlInputElement};

use crate::{
    event::Event,
    state::{self},
};

pub fn close_all() {
    let dialogs = document().get_elements_by_tag_name("dialog");

    for i in 0..dialogs.length() {
        let dialog_item = dialogs.item(i);

        let Some(dialog) = dialog_item
            .as_ref()
            .and_then(|el| el.dyn_ref::<HtmlDialogElement>())
        else {
            unreachable!()
        };

        dialog.set_open(false);
    }
}

pub fn activate(element_id: Id, prompt: Prompt) {
    let dialogs = document().get_elements_by_tag_name("dialog");

    for i in 0..dialogs.length() {
        let dialog_item = dialogs.item(i);

        let Some(dialog) = dialog_item
            .as_ref()
            .and_then(|el| el.dyn_ref::<HtmlDialogElement>())
        else {
            unreachable!()
        };

        dialog.set_open(true);

        let forms = dialog.get_elements_by_tag_name("form");

        if let Some(form_item) = forms.item(0) {
            let Some(form) = form_item.dyn_ref::<HtmlFormElement>() else {
                unreachable!()
            };

            let Some(text) =
                form_item.get_elements_by_tag_name("input").item(0)
            else {
                log::warn!("Form has no input element.");
                continue;
            };

            let text: HtmlInputElement = text.dyn_into().unwrap();
            let _ = text.focus();

            EventListener::once(form, "submit", move |_| {
                state::handle_event(Event::PromptResponse {
                    element_id,
                    response: PromptResponse::Text {
                        response: text.value(),
                        metadata: prompt.metadata(),
                    },
                });

                let Some(dialog) = dialog_item
                    .as_ref()
                    .and_then(|el| el.dyn_ref::<HtmlDialogElement>())
                else {
                    return;
                };

                dialog.set_open(false);
            })
            .forget();
            return;
        }
    }
}
