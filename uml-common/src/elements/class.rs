use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    drawable::Drawable,
    interaction::{InteractionState, Interactive},
    prompt::{Prompt, PromptResponse},
    stroke::Stroke,
};

use super::{Label, Rectangle, TextProperties};

const DEFAULT_COLOR: Color = const {
    Color::Rgb {
        red: 244,
        green: 244,
        blue: 244,
    }
};

const DEFAULT_STROKE: Stroke = const {
    Stroke::new(
        2,
        Color::Rgb {
            red: 210,
            green: 210,
            blue: 210,
        },
    )
};

const HIGHLIGHT_STROKE: Stroke = const {
    Stroke::new(
        2,
        Color::Rgb {
            red: 142,
            green: 202,
            blue: 230,
        },
    )
};

const TITLE_COLOR: Color = const {
    Color::Rgb {
        red: 31,
        green: 31,
        blue: 31,
    }
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    x: i32,
    y: i32,

    color: Color,
    radius: Option<u32>,
    stroke: Option<Stroke>,
    margin: u32,

    title: Option<Label>,

    #[serde(skip)]
    interaction_state: InteractionState,
}

impl Class {
    pub fn new(
        x: i32,
        y: i32,
        color: Option<Color>,
        stroke: Option<Stroke>,
        radius: Option<u32>,
    ) -> Self {
        let color = color.unwrap_or(DEFAULT_COLOR);

        Self {
            x,
            y,
            color,
            radius,
            stroke,
            margin: 20,
            title: None,
            interaction_state: InteractionState::default(),
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> Option<u32> {
        self.title
            .as_ref()
            .and_then(|t| t.width())
            .map(|w| w + 2 * self.margin)
    }

    pub fn height(&self) -> Option<u32> {
        self.title
            .as_ref()
            .and_then(|t| t.height())
            .map(|h| h + 2 * self.margin)
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn radius(&self) -> Option<u32> {
        self.radius
    }

    pub fn stroke(&self) -> Option<Stroke> {
        self.stroke
    }
}

impl Drawable for Class {
    fn initalize(&mut self, canvas: &impl Canvas) {
        match self.title.as_mut() {
            Some(t) => t.initalize(canvas),
            None => {
                let props = TextProperties::new(20.0, "sans-serif");
                self.title = Some(Label::new(
                    self.x + self.margin as i32,
                    self.y + self.margin as i32,
                    "Test class",
                    props,
                    TITLE_COLOR,
                ));
            }
        }
    }

    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        let Some(title) = &self.title else {
            log::error!("Cannot draw uninitialized class.");
            return;
        };

        let Some(width) = title.width() else {
            log::error!("Class title does not have a width.");
            return;
        };

        let Some(height) = title.height() else {
            log::error!("Class title does not have a height.");
            return;
        };

        let stroke = if self.is_hovered() {
            HIGHLIGHT_STROKE
        } else {
            DEFAULT_STROKE
        };

        let bg = Rectangle::new(
            self.x,
            self.y,
            width + 2 * self.margin,
            height + 2 * self.margin,
            DEFAULT_COLOR,
            Some(2),
            Some(stroke),
        );

        bg.draw(canvas, camera);
        title.draw(canvas, camera);
    }
}

impl Interactive for Class {
    fn get_interaction(&self) -> InteractionState {
        self.interaction_state
    }

    fn get_interaction_mut(&mut self) -> &mut InteractionState {
        &mut self.interaction_state
    }

    fn adjust_position(&mut self, delta_x: i32, delta_y: i32) {
        self.x += delta_x;
        self.y += delta_y;

        if let Some(title) = &mut self.title {
            title.adjust_position(delta_x, delta_y);
        }
    }

    #[allow(unused_variables)]
    fn click(&mut self, x: i32, y: i32) -> Option<Prompt> {
        let value = self
            .title
            .as_ref()
            .map(|t| t.text())
            .unwrap_or("")
            .to_string();

        Some(Prompt::Text {
            explanation: "Provide this class with a new name".into(),
            placeholder: "Class name".into(),
            value,
            metadata: Rc::new(()),
        })
    }

    fn prompt(&mut self, response: PromptResponse) {
        let PromptResponse::Text {
            response,
            metadata: _,
        } = response;

        if let Some(title) = self.title.as_mut() {
            title.set_text(response);
        }
    }
}
