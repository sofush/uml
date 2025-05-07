use html_canvas::HtmlCanvas;
use log::Level;
use uml_common::{draw_context::Canvas, elements::Rectangle};
use wasm_bindgen::prelude::*;

mod html_canvas;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).unwrap();
    let canvas = HtmlCanvas::new();
    canvas.draw_rectangle(Rectangle {});
    Ok(())
}
