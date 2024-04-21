use std::{rc::Rc, sync::Mutex};

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
    fn get_name(&self) -> &'static str;
    fn add_child(&mut self, child: UIElementRef);
    fn dump(&self, indent: i32);
    fn add_content_string(&mut self, s: String);
    fn get_attribute(&self, s: &String) -> Option<String>;
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

    pub fn get_attribute(&self, s: &String) -> Option<String> {
        for attr in self.attributes.iter() {
            if attr.name.local_name.eq(s) {
                return Option::Some(attr.value.clone());
            }
        }
        Option::None
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