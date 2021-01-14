use js_sys::{Function};
use wasm_bindgen::prelude::*;

use fastann::demo::demo;

#[wasm_bindgen]
pub fn run_demo(callback: &Function) {
    let result = demo::run_demo();
    let args = JsValue::from(&result);
    callback.call1(&JsValue::null(), &args).expect("Error");
}
