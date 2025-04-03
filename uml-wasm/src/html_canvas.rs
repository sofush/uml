use uml_common::{canvas::Canvas, elements::Rectangle};
use wasm_bindgen::JsCast;

#[derive(Clone)]
pub struct HtmlCanvas {
    element: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
}

impl HtmlCanvas {
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let element: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Self { element, context }
    }

    pub fn update_size(&self) {
        let new_height = self.element.offset_height();
        let new_width = self.element.offset_width();
        self.element.set_height(new_height as u32);
        self.element.set_width(new_width as u32);
    }
}

impl Canvas for HtmlCanvas {
    fn draw_rectangle(&self, rect: Rectangle) {
        self.context.set_fill_style_str(&rect.color().to_string());
        self.context.fill_rect(
            rect.x() as f64,
            rect.y() as f64,
            rect.width() as f64,
            rect.height() as f64,
        );
    }
}
