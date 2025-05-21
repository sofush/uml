use std::{any::Any, rc::Rc};

#[derive(Debug, Clone)]
pub enum Prompt {
    Text {
        explanation: String,
        placeholder: String,
        value: String,
        metadata: Rc<dyn Any>,
    },
}

#[derive(Debug, Clone)]
pub enum PromptResponse {
    Text {
        response: String,
        metadata: Rc<dyn Any>,
    },
}

impl Prompt {
    pub fn metadata(&self) -> Rc<dyn Any> {
        match self {
            Prompt::Text { metadata, .. } => metadata.clone(),
        }
    }
}
