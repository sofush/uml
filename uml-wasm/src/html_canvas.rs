use uml_common::{draw_context::Canvas, elements::Rectangle};
use wasm_bindgen::JsCast;

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
}

impl Canvas for HtmlCanvas {
    fn draw_rectangle(&self, rect: Rectangle) {
        self.context.set_fill_style_str("rgb(200 0 0)");
        self.context.fill_rect(10.0, 10.0, 50.0, 50.0);

        self.context.set_fill_style_str("rgb(0 0 200 / 50%)");
        self.context.fill_rect(30.0, 30.0, 50.0, 50.0);
    }
}
