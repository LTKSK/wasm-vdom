use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, Element, HtmlElement, ImageData, Request, RequestInit,
    RequestMode, Response, Window,
};

#[wasm_bindgen]
pub fn greeting() -> String {
    "hello world".to_string()
}

//#[derive(Debug)]
//pub struct Props {
//    title: String,
//    children: Element
//}
//
//#[derive(Debug)]
//pub struct Element {
//    type: String,
//    props: Props,
//}

#[wasm_bindgen]
pub fn render(container: &JsValue) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    Ok(())
}
