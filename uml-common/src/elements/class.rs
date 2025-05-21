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

const TEXT_COLOR: Color = const {
    Color::Rgb {
        red: 31,
        green: 31,
        blue: 31,
    }
};

const MARGIN: u32 = 20;
const SPACING: u32 = 16;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    x: i32,
    y: i32,

    color: Color,
    radius: Option<u32>,
    stroke: Option<Stroke>,

    title: Label,
    attributes: Vec<Label>,

    #[serde(skip)]
    interaction_state: InteractionState,
}

impl Class {
    pub fn new(
        x: i32,
        y: i32,
        name: String,
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
            interaction_state: InteractionState::default(),
            attributes: vec![
                Label::new(
                    0,
                    0,
                    "Attribute 1",
                    TextProperties::default(),
                    TEXT_COLOR,
                ),
                Label::new(
                    0,
                    0,
                    "Attribute 2",
                    TextProperties::default(),
                    TEXT_COLOR,
                ),
            ],
            title: Label::new(
                0,
                0,
                name,
                TextProperties::default().weight(700),
                TEXT_COLOR,
            ),
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> Option<u32> {
        let mut w = self.title.width().map(|w| w + 2 * MARGIN)?;

        for attribute in &self.attributes {
            w = u32::max(w, attribute.width()? + 2 * MARGIN);
        }

        Some(w)
    }

    pub fn height(&self) -> Option<u32> {
        let mut h = self.title.height().map(|h| h + 2 * MARGIN)?;

        for attribute in &self.attributes {
            h += SPACING;
            h += attribute.height()?;
        }

        Some(h)
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
        self.title.initalize(canvas);

        let mut offset_y = self.y + MARGIN as i32;

        self.title.set_position(self.x + MARGIN as i32, offset_y);

        offset_y += self.title.height().unwrap_or(0) as i32;
        offset_y += SPACING as i32;

        for attribute in &mut self.attributes {
            attribute.initalize(canvas);
            attribute.set_position(self.x + MARGIN as i32, offset_y);
            offset_y += attribute.height().unwrap_or(0) as i32 + SPACING as i32;
        }
    }

    fn draw(&self, canvas: &impl Canvas, camera: &Camera) {
        let Some(width) = self.width() else {
            log::error!("Unable to draw class, class does not have a width.");
            return;
        };

        let Some(height) = self.height() else {
            log::error!("Unable to draw class, class does not have a height.");
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
            width,
            height,
            DEFAULT_COLOR,
            Some(2),
            Some(stroke),
        );

        bg.draw(canvas, camera);
        self.title.draw(canvas, camera);

        for attribute in &self.attributes {
            attribute.draw(canvas, camera);
        }
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
        self.title.adjust_position(delta_x, delta_y);

        for attribute in &mut self.attributes {
            attribute.adjust_position(delta_x, delta_y);
        }
    }

    #[allow(unused_variables)]
    fn click(&mut self, x: i32, y: i32) -> Option<Prompt> {
        let value = self.title.text().to_string();

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

        if response.is_empty() {
            return;
        }

        self.title.set_text(response);
    }
}
