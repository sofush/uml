use log::{Level, debug};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Debug).unwrap();
    debug!("Hello, world!");
    Ok(())
}
