use std::{i32, sync::Arc};

use parking_lot::Mutex;
use std::collections::HashMap;

use crate::{events::Event, visitor::Visitor};

pub type UIElementRef = Arc<Mutex<dyn UIElement>>;

pub struct UITree {
    pub root: Option<UIElementRef>,
}

impl UITree {
    pub fn new() -> UITree {
        UITree { root: Option::None }
    }

    pub fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        if let Some(x) = &self.root {
            let k = x.lock();
            if k.get_id() == id {
                return Some(x.clone());
            }

            return k.find_by_id(id);
        }
        return None;
    }
}

pub type UITreeRef = &'static UITree;

// utility func for extracting props from XAML
pub fn get_attribute(
    attributes: &HashMap<String, String>,
    name: &str,
    default_str: &str,
) -> String {
    if let Some(ret) = attributes.get(name) {
        return ret.to_string();
    }

    default_str.to_string()
}

pub trait UIAlloc {
    fn new(attributes: &HashMap<String, String>, id: String) -> Self;
}

pub trait UIElement {
    fn get_id(&self) -> String;
    fn find_by_id(&self, id: String) -> Option<UIElementRef>;

    fn get_ui_type_name(&self) -> &'static str;
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef);
    fn set_parent(&mut self, parent: UIElementRef);
    fn dump(&self, indent: i32);
    fn add_content_string(&mut self, s: String);
    fn get_attribute(&self, s: &str) -> Option<&String>;

    fn visit(&self, visitor: &mut dyn Visitor);

    fn handle_event(&self, ev: Event);
}

pub struct UICommon {
    parent: Option<UIElementRef>,
    attributes: HashMap<String, String>,
    children: Vec<UIElementRef>,
    id: String,
    parent_type: String,
}

impl UICommon {
    pub fn new(attributes: &HashMap<String, String>, parent_type: &str, id: String) -> UICommon {
        UICommon {
            parent: Option::None,
            attributes: attributes.clone(),
            children: Vec::new(),
            id,
            parent_type: parent_type.to_string(),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        for c in self.children.iter() {
            let k = c.clone();
            if k.lock().get_id() == id {
                return Some(k);
            }
        }

        return None;
    }

    pub fn set_parent(&mut self, parent: UIElementRef) {
        self.parent = Some(parent);
    }

    pub fn handle_event(&self, ev: Event) {
        if let Some(p) = self.internal_get_attribute(&ev.get_name()) {
            let mc = ev.get_callable();
            mc.lock().call_method(p.as_str());
        }

        if let Some(p) = &self.parent {
            p.lock().handle_event(ev);
        }
    }

    fn get_prop_name(&self, prop: &str) -> String {
        format!("{}.{}", self.parent_type.to_string(), prop)
    }

    pub fn get_attr_opt(&self, prop_name: &str) -> Option<&String> {
        let pn = self.get_prop_name(prop_name);
        self.internal_get_attribute(pn.as_str())
    }

    pub fn get_attr(&self, prop_name: &str) -> String {
        let x = self.get_attr_opt(prop_name);
        if x.is_none() {
            return "".to_string();
        }
        x.unwrap().to_string()
    }

    pub fn set_attr(&mut self, prop: &str, value: String) {
        let pn = self.get_prop_name(prop);
        self.internal_set_attribute(pn.as_str(), &value)
    }


    pub fn set_width(&mut self, v: i32) {
        self.set_attr("Width", v.to_string());
    }

    pub fn set_height(&mut self, v: i32) {
        self.set_attr("Height", v.to_string());
    }

    pub fn get_width(&self) -> Option<i32> {
        let pn = self.get_prop_name("Width");
        let value = get_attribute(&self.attributes, pn.as_str(), "").parse::<i32>();
        if value.is_err() {
            return Option::None;
        }
        Option::Some(value.unwrap())
    }

    pub fn get_height(&self) -> Option<i32> {
        let pn = self.get_prop_name("Height");

        let value = get_attribute(&self.attributes, pn.as_str(), "").parse::<i32>();
        if value.is_err() {
            return Option::None;
        }
        Option::Some(value.unwrap())
    }

    pub fn visit(&self, visitor: &mut dyn Visitor) {
        for c in self.children.iter() {
            let k = c.clone();
            k.lock().visit(visitor);
        }
    }

    fn internal_get_attribute(&self, s: &str) -> Option<&String> {
        return self.attributes.get(s);
    }

    fn internal_set_attribute(&mut self, property_name: &str, new_value: &String) {
        self.attributes
            .insert(property_name.to_string(), new_value.to_string());
    }

    pub fn add_child(&mut self, child: UIElementRef, me: UIElementRef) {
        self.children.push(child.clone());
        child.lock().set_parent(me.clone());
    }

    pub fn dump(&self, _indent: i32) {
        let indent = 1 + _indent;
        for c in self.children.iter() {
            let l = c.lock();
            l.dump(indent);
        }
    }
}

pub fn tabs(c: i32) -> String {
    let mut k: String = String::new();
    for _ in 0..c {
        k.push_str("   ");
    }
    k
}
