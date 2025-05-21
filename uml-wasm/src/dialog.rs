use std::cell::RefCell;

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

thread_local! {
    pub static SHARED_DIALOG: RefCell<Dialog> = const { RefCell::new(Dialog::new()) };
}

#[derive(Default)]
pub struct Dialog {
    listener: Option<EventListener>,
}

impl Dialog {
    pub const fn new() -> Self {
        Self { listener: None }
    }

    pub fn deactivate(&mut self) {
        self.listener = None;
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

    pub fn activate(&mut self, element_id: Id, prompt: Prompt) {
        let Prompt::Text {
            explanation,
            placeholder,
            value,
            metadata,
        } = prompt;
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

            if let Some(el) =
                dialog.get_elements_by_class_name("explanation").item(0)
            {
                el.set_inner_html(&explanation);
            }

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

                text.set_placeholder(&placeholder);
                text.set_value(&value);

                let listener = EventListener::once(form, "submit", move |_| {
                    state::handle_event(Event::PromptResponse {
                        element_id,
                        response: PromptResponse::Text {
                            response: text.value(),
                            metadata,
                        },
                    });

                    SHARED_DIALOG.with_borrow_mut(|d| d.deactivate());
                });

                self.listener = Some(listener);
                return;
            }
        }
    }

    pub fn is_active(&self) -> bool {
        self.listener.is_some()
    }
}
