use std::fmt::Display;

use rand::{Rng, distributions::Alphanumeric, thread_rng};
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
    pub id: String,
}

impl Element {
    pub fn random() -> Self {
        thread_rng().r#gen()
    }
}

impl From<Vec<Element>> for Diagram {
    fn from(elements: Vec<Element>) -> Self {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        Self { elements, id: s }
    }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}
