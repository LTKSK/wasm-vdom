use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen]
pub fn greeting() -> String {
    "hello world".to_string()
}

#[derive(Debug)]
enum NodeType {
    VNODE,
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
    value: Option<String>,
    children: Vec<VNode>,
}

fn create_element(node_type: NodeType, props: Props, children: Vec<VNode>) -> Option<VNode> {
    // まずは決め打ちでElementを作って、これをrender関数にぶん投げたら
    // domができる
    // という形を実装して、その後ちゃんとした関数にする
    // それもできたら差分チェックを作る
    // 最初はdivとtextだけ
    Some(VNode {
        node_type,
        props,
        value: None,
        children: vec![VNode {
            node_type: NodeType::VNODE,
            props: Props::new(),
            value: Some("testだよん".to_string()),
            children: vec![],
        }],
    })
}

fn render_dom(contaner: &JsValue, root: VNode) -> Result<(), JsValue> {
    // 受け取ったVNodeをもとに、DOMツリーを構築する
    // create_element("div", Props::new("id".to_string()): })
    Ok(())
}

#[wasm_bindgen]
pub fn render(id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    // containerのdomは引数として既にある状態
    // propsの情報を、dom[name] = valueとして詰めていかないといけない
    // containerに対して、childrenをappendChildしていく必要がある
    // create_element("div", Props::new("id".to_string()): })
    let node = document.create_element("div")?;
    node.set_inner_html("append test");
    let container = document.get_element_by_id(id).expect("no id `container`");
    container.append_child(&node)?;
    Ok(())
}
