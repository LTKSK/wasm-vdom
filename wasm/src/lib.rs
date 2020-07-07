use std::{cell::RefCell, rc::Rc};
extern crate wasm_bindgen;
use wasm_bindgen::{prelude::*, JsCast};
extern crate web_sys;
use web_sys::Node;

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

enum EventHandler {
    OnClick,
}

struct VNode {
    vnode_type: VNodeType,
    node_type: NodeType,
    value: &'static str,
    attributes: Vec<(String, String)>,
    //event_handlers: Vec<(EventHandler, Closure<dyn FnMut(web_sys::Event)>)>,
    //event_handlers: Vec<(EventHandler, Box<dyn FnMut(web_sys::Event)>)>,
    children: Vec<VNode>,
}

fn create_element(vnode: &VNode) -> web_sys::Node {
    // 受け取ったVNodeをもとに、DOMツリーを構築する
    let document = web_sys::window().unwrap().document().unwrap();
    match vnode.vnode_type {
        VNodeType::TextElement => {
            let element = document.create_text_node(&vnode.value);
            element.into()
        }
        VNodeType::Element => {
            let element = document.create_element(&vnode.node_type.name()).unwrap();
            element.set_inner_html(&vnode.value);
            //attributesの適用
            for (key, value) in &vnode.attributes {
                element.set_attribute(&key, &value).unwrap();
            }
            for child in &vnode.children {
                element.append_child(&create_element(&child)).unwrap();
            }
            element.into()
        }
    }
}

fn request_idle_callback(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_idle_callback(f.as_ref().unchecked_ref())
        .expect("should register `requestIdleCallback` ");
}

pub trait Component: 'static {
    type State;
    fn render(&mut self, id: &str) -> Result<(), JsValue>;
    fn get_state(&self) -> &Self::State;
    fn set_state(&mut self, state: Self::State);
}

struct AppState {
    counter: i32,
}

type State = Rc<RefCell<AppState>>;
struct App {
    state: AppState,
}
impl Component for App {
    type State = AppState;

    fn render(&mut self, id: &str) -> Result<(), JsValue> {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let container = document.get_element_by_id(id).expect("no id `container`");
        //let closure = Closure::wrap(Box::new(move |event: web_sys::Event| {
        //    &self.set_state(Self::State {
        //        counter: &self.state.counter + 1,
        //    });
        //    web_sys::console::log_1(&"update state".into());
        //}) as Box<dyn FnMut(_)>);

        let vnode = Rc::new(RefCell::new(VNode {
            vnode_type: VNodeType::Element,
            node_type: NodeType::Button,
            value: "ボタンだよ",
            attributes: vec![],
            children: vec![],
        }));

        let vnode = vnode.clone();
        // Option<Closure>を後で入れる
        let f = Rc::new(RefCell::new(None));
        // cloneで手に入るのはRefCellの参照
        let g = f.clone();
        // NoneでRefCellを作っているのでSomeでくくる
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let v = &vnode.borrow();
            let el = create_element(v);
            //el.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            match container.first_child() {
                Some(first_child) => container.replace_child(&el, &first_child).unwrap(),
                None => container.append_child(&el).unwrap(),
            };
            request_idle_callback(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_idle_callback(g.borrow().as_ref().unwrap());
        Ok(())
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state
    }
    fn get_state(&self) -> &Self::State {
        &self.state
    }
}

#[wasm_bindgen]
pub fn render(id: &str) -> Result<(), JsValue> {
    let mut app = App {
        state: AppState { counter: 0 },
    };
    app.render(id);
    Ok(())
}
