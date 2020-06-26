extern crate wasm_bindgen;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
extern crate web_sys;
use web_sys::{Document, Element, HtmlElement, Node, Window};

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
    Button,
}

impl NodeType {
    pub fn name(&self) -> &str {
        match self {
            Self::Div => "div",
            Self::Button => "button",
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

fn create_element(vnode: VNode) -> web_sys::Node {
    // 受け取ったVNodeをもとに、DOMツリーを構築する
    let document = web_sys::window().unwrap().document().unwrap();
    match vnode.vnode_type {
        VNodeType::TextElement => {
            let element = document.create_text_node(&vnode.value);
            element.into()
        }
        VNodeType::Element => {
            let element = document.create_element(&vnode.node_type.name()).unwrap();
            for child in vnode.children {
                element.append_child(&create_element(child));
            }
            element.into()
        }
    }
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
                vnode_type: VNodeType::Element,
                props: Props::new(),
                node_type: NodeType::Div,
                value: "child2だよん".to_string(),
                children: vec![VNode {
                    vnode_type: VNodeType::Element,
                    props: Props::new(),
                    node_type: NodeType::Div,
                    value: "".to_string(),
                    children: vec![
                        VNode {
                            vnode_type: VNodeType::TextElement,
                            props: Props::new(),
                            node_type: NodeType::Div,
                            value: "child2のchild2だよん".to_string(),
                            children: vec![],
                        },
                        VNode {
                            vnode_type: VNodeType::Element,
                            props: Props::new(),
                            node_type: NodeType::Button,
                            value: "".to_string(),
                            children: vec![],
                        },
                        VNode {
                            vnode_type: VNodeType::TextElement,
                            props: Props::new(),
                            node_type: NodeType::Div,
                            value: "child2のchild2だよん".to_string(),
                            children: vec![],
                        },
                    ],
                }],
            },
        ],
    };
    container.append_child(&create_element(vnode))?;
    Ok(())
}
