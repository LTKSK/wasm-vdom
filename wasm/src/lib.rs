extern crate wasm_bindgen;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
extern crate web_sys;
//use web_sys::{Document, Element, HtmlElement, Window};

#[wasm_bindgen]
pub fn greeting() -> String {
    "hello world".to_string()
}

// text単体もしくは何かしらのElementの二種類のみ
#[derive(Debug)]
enum VNodeType {
    Element,
    TextElement,
}

#[derive(Debug)]
enum NodeType {
    Div,
}

impl NodeType {
    pub fn name(&self) -> &str {
        match self {
            Self::Div => "div",
        }
    }
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
    vnode_type: VNodeType,
    node_type: NodeType,
    props: Props,
    value: String,
    children: Vec<VNode>,
}

//fn create_element(vnode_type: VNodeType, props: Props, children: Vec<VNode>) ->
//Option<web_sys::Node> {
//    // まずは決め打ちでElementを作って、これをrender関数にぶん投げたら
//    // domができる
//    // という形を実装して、その後ちゃんとした関数にする
//    // それもできたら差分チェックを作る
//    // 最初はdivとtextだけ
//    Some(VNode {
//        vnode_type,
//        props,
//        node_type: NodeType::Div,
//        value: "".to_string(),
//        children: vec![VNode {
//            vnode_type: VNodeType::Element,
//            props: Props::new(),
//            node_type: NodeType::Div,
//            value: "testだよん".to_string(),
//            children: vec![],
//        }],
//    })
//}

fn render_dom(contaner: &JsValue, root: VNode) -> Result<(), JsValue> {
    // 受け取ったVNodeをもとに、DOMツリーを構築する
    // create_element("div", Props::new("id".to_string()): })
    Ok(())
}

#[wasm_bindgen]
pub fn render(id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.get_element_by_id(id).expect("no id `container`");
    let vnode = VNode {
        vnode_type: VNodeType::Element,
        node_type: NodeType::Div,
        props: Props::new(),
        value: "".to_string(),
        children: vec![
            VNode {
                vnode_type: VNodeType::TextElement,
                props: Props::new(),
                node_type: NodeType::Div,
                value: "child1だよん".to_string(),
                children: vec![],
            },
            VNode {
                vnode_type: VNodeType::TextElement,
                props: Props::new(),
                node_type: NodeType::Div,
                value: "child2だよん".to_string(),
                children: vec![],
            },
        ],
    };

    // containerのdomは引数として既にある状態
    // propsの情報を、dom[name] = valueとして詰めていかないといけない
    // containerに対して、childrenをappendChildしていく必要がある
    let node = document.create_element(&vnode.node_type.name())?;
    for child in &vnode.children {
        match child.vnode_type {
            VNodeType::Element => {
                let child_node = document.create_element(child.node_type.name())?;
                child_node.set_inner_html(&child.value);
                node.append_child(&child_node)?;
            }
            VNodeType::TextElement => {
                // なぜかcreate_text_nodeが見つからない
                let child_node = document.create_element("text")?;
                child_node.set_text_content(Some(&child.value));
                node.append_child(&child_node)?;
            }
        }
    }
    container.append_child(&node)?;
    Ok(())
}
