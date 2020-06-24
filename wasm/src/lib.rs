use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen]
pub fn greeting() -> String {
    "hello world".to_string()
}

#[derive(Debug)]
enum NodeType {
    TEXT_ELEMENT,
}

#[derive(Debug)]
enum NodeValue {
    number(f32),
    string(String),
    boolean(bool),
}

type Props = HashMap<String, NodeValue>;

#[derive(Debug)]
struct VNode {
    node_type: NodeType,
    props: Props,
    //realDOMへの参照?
    //children?
    value: String,
    children: Vec<VNode>,
}

fn create_element(node_type: NodeType, props: Props, children: Vec<VNode>) -> Option<VNode> {
    Some(VNode {
        node_type,
        props,
        value: "hoge".to_string(),
        children: vec![],
    })
}

#[wasm_bindgen]
pub fn render(container: &JsValue) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    // containerのdomは引数として既にある状態
    // propsの情報を、dom[name] = valueとして詰めていかないといけない
    // containerに対して、childrenをappendChildしていく必要がある
    Ok(())
}
