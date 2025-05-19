use uml_common::{
    camera::Camera, elements::Element, id::Id, interaction::Interactive,
};

use crate::event::Event;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HoverHandler {
    id: Option<Id>,
}

impl HoverHandler {
    pub fn handle(
        &mut self,
        event: &Event,
        elements: &mut [Element],
        camera: &Camera,
    ) {
        let Event::Mouse(event) = event else {
            return;
        };

        let x = event.x() + camera.x() as i32;
        let y = event.y() + camera.y() as i32;

        let hovered = elements
            .iter_mut()
            .rev()
            .find(|e| e.cursor_intersects(x, y));

        if let Some(hovered) = hovered {
            if Some(hovered.id()) == self.id {
                return;
            }

            hovered.hover_enter();
            let id = Some(hovered.id());

            if let Some(previous_hovered) =
                elements.iter_mut().find(|el| Some(el.id()) == self.id)
            {
                previous_hovered.hover_leave();
            }

            self.id = id;
        } else if let Some(previous_id) = self.id.take() {
            let Some(previous_hovered) =
                elements.iter_mut().find(|el| el.id() == previous_id)
            else {
                return;
            };

            previous_hovered.hover_leave();
            self.id = None;
        }
    }
}
