use std::{i32, sync::Arc};

use parking_lot::Mutex;
use xml::{attribute::OwnedAttribute, name::OwnedName};

use crate::{events::Event, visitor::Visitor};

pub type UIElementRef = Arc<Mutex<dyn UIElement>>;

pub struct UITree {
    pub root : Option<UIElementRef>
}

impl UITree {
    pub fn new() -> UITree {
        UITree {
            root : Option::None
        }
    }

    pub fn trigger_event(&self) {
        println!("need to trigger event");
    }
}

pub type UITreeRef = &'static UITree;

// utility func for extracting props from XAML
pub fn get_attribute(
    attributes: &[xml::attribute::OwnedAttribute],
    name: &str,
    default_str: &str,
) -> String {
    for a in attributes.iter() {
        if a.name.local_name == name {
            return a.value.to_string();
        }
    }
    default_str.to_string()
}

pub trait UIAlloc {
    fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Self;
}

pub trait UIElement {
    fn get_ui_type_name(&self) -> &'static str;
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef);
    fn set_parent(&mut self, parent: UIElementRef);
    fn dump(&self, indent: i32);
    fn add_content_string(&mut self, s: String);
    fn get_attribute(&self, s: &str) -> Option<String>;

    fn visit(&self, visitor: &mut dyn Visitor);

    fn handle_event(&self, ev: Event);
}

pub struct UICommon {
    parent: Option<UIElementRef>,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    _width: String,
    _height: String,
    children: Vec<UIElementRef>,
}

impl UICommon {
    pub fn new( attributes: Vec<xml::attribute::OwnedAttribute>) -> UICommon {
        UICommon {
            parent: Option::None,
            attributes: attributes.clone(),
            children: Vec::new(),
            _width: get_attribute(&attributes, "Width", ""),
            _height: get_attribute(&attributes, "Height", ""),
        }
    }

    pub fn set_parent(&mut self, parent: UIElementRef) {
        self.parent = Some(parent);
    }

    pub fn handle_event(&self, ev: Event)
    {
        if let Some(p) = &self.parent {
            p.lock().handle_event(ev);
        }
    }

    pub fn set_width(&mut self, v: i32) {
        self.set_attribute(&"Width".to_string(), &v.to_string())
    }

    pub fn set_height(&mut self, v: i32) {
        self.set_attribute(&"Height".to_string(), & v.to_string())
    }

    pub fn get_width(&self) -> Option<i32> { 
       let value = get_attribute(&self.attributes, "Width", "").parse::<i32>();
       if value.is_err() {
        return Option::None;
       }
        Option::Some(value.unwrap())
    }
    
    pub fn get_height(&self) -> Option<i32> {
        let value = get_attribute(&self.attributes, "Height", "").parse::<i32>();
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

    pub fn get_attribute(&self, s: &str) -> Option<String> {
        for attr in self.attributes.iter() {
            if attr.name.local_name.eq(s) {
                return Option::Some(attr.value.clone());
            }
        }
        Option::None
    }

    pub fn set_attribute(&mut self, property_name: &String, new_value: &String) {
        for attr in self.attributes.iter_mut() {
            if attr.name.local_name.eq(property_name) {
                attr.value = new_value.clone();
                return;
            }
        }

        let owned_name = OwnedName::local(property_name);
        let value = OwnedAttribute::new(owned_name, new_value);
        self.attributes.push(value);
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
