use uml_common::{
    camera::Camera,
    canvas::Canvas,
    elements::{Label, Rectangle, TextProperties},
    size::Size,
};
use wasm_bindgen::JsCast;
use web_sys::TextMetrics;

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
    fn draw_rectangle(&self, rect: Rectangle, camera: &Camera) {
        self.context.set_fill_style_str(&rect.color().to_string());
        self.context.fill_rect(
            rect.x() as f64 - camera.x(),
            rect.y() as f64 - camera.y(),
            rect.width() as f64,
            rect.height() as f64,
        );
    }

    fn draw_text(&self, label: &Label, camera: &Camera) {
        self.context.set_fill_style_str(&label.color().to_string());
        self.context.set_font(&label.props().get_font_string());

        let x = label.x() as f64 - camera.x();
        let y = label.y() as f64 - camera.y();

        if self.context.fill_text(label.text(), x, y).is_err() {
            log::debug!("Call to fill_text() failed.")
        }
    }

    fn measure_text(
        &self,
        text: &str,
        props: &TextProperties,
    ) -> Option<Size<f32>> {
        self.context.set_font(&props.get_font_string());

        let Ok(ret) = self.context.measure_text(text) else {
            log::error!("Could not measure text.");
            return None;
        };

        let Some(metrics) = ret.dyn_ref::<TextMetrics>() else {
            log::error!("Could not convert return type into TextMetrics.");
            return None;
        };

        let height = metrics.actual_bounding_box_ascent()
            + metrics.actual_bounding_box_descent();
        let width = metrics.actual_bounding_box_left()
            + metrics.actual_bounding_box_right();
        let size = Size::new(width as _, height as _);

        Some(size)
    }
}
