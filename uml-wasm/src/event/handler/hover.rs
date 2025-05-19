use uml_common::{camera::Camera, elements::Element, id::Id};

use crate::event::{Event, Outcome};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HoverHandler {
    prev_hovered: Option<Id>,
}

impl HoverHandler {
    pub fn handle(
        &mut self,
        event: &Event,
        elements: &mut [Element],
        camera: &Camera,
    ) -> Vec<Outcome> {
        let Event::Mouse(event) = event else {
            return vec![];
        };

        let x = event.x() + camera.x() as i32;
        let y = event.y() + camera.y() as i32;

        let hovered = elements
            .iter_mut()
            .rev()
            .find(|e| e.cursor_intersects(x, y))
            .map(|e| e.id());

        self.get_outcomes(hovered)
    }

    fn get_outcomes(&mut self, hovered: Option<Id>) -> Vec<Outcome> {
        if hovered == self.prev_hovered {
            return vec![];
        }

        let mut outcomes = vec![];

        if let Some(prev) = self.prev_hovered {
            outcomes.push(Outcome::HoverElement {
                id: prev,
                hovered: false,
            });
        }

        if let Some(hovered) = hovered {
            outcomes.push(Outcome::HoverElement {
                id: hovered,
                hovered: true,
            });
        }

        self.prev_hovered = hovered;
        outcomes
    }
}
