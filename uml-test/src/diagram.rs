use rand::{Rng, thread_rng};
use rand_derive2::RandGen;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RandGen)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RandGen)]
pub enum Element {
    Rectangle { x: i32, y: i32 },
    Circle { color: Color, size: f32 },
    Text { text: String },
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Diagram {
    pub elements: Vec<Element>,
}

impl Element {
    pub fn random() -> Self {
        thread_rng().r#gen()
    }
}

impl From<Vec<Element>> for Diagram {
    fn from(elements: Vec<Element>) -> Self {
        Self { elements }
    }
}
