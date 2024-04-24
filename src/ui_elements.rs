use std::{i32, rc::Rc, sync::Mutex};

use xml::{attribute::OwnedAttribute, name::OwnedName};

use crate::visitor::Visitor;

pub type UIElementRef = Rc<Mutex<dyn UIElement>>;

// utility func for extracting props from XAML
pub fn get_attribute(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
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

pub trait UIElement {
    fn get_ui_type_name(&self) -> &'static str;
    fn add_child(&mut self, child: UIElementRef);
    fn dump(&self, indent: i32);
    fn add_content_string(&mut self, s: String);
    fn get_attribute(&self, s: &String) -> Option<String>;

    fn visit(&self, visitor: &mut dyn Visitor);
}

pub struct UICommon {
    attributes: Vec<xml::attribute::OwnedAttribute>,
    width: String,
    height: String,
    children: Vec<UIElementRef>,
}

impl UICommon {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> UICommon {
        UICommon {
            attributes: attributes.clone(),
            children: Vec::new(),
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
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
       return Option::Some(value.unwrap());
    }
    
    pub fn get_height(&self) -> Option<i32> {
        let value = get_attribute(&self.attributes, "Height", "").parse::<i32>();
        if value.is_err() {
            return Option::None;
        }
        return Option::Some(value.unwrap());
    }

    pub fn visit(&self, visitor: &mut dyn Visitor) {
        for c in &self.children {
            c.as_ref().lock().unwrap().visit(visitor);
        }
    }

    pub fn get_attribute(&self, s: &String) -> Option<String> {
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

    pub fn add_child(&mut self, child: UIElementRef) {
        self.children.push(child)
    }

    pub fn dump(&self, _indent: i32) {
        let indent = 1 + _indent;
        for c in self.children.iter() {
            let l = c.lock();
            l.unwrap().dump(indent);
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
