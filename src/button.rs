use crate::ui_elements::{tabs, UICommon, UIElement, UIElementRef};


pub struct Button {
    common: UICommon,
}

impl Button {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Button {
        Button {
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for Button {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_name(&self) -> &'static str {
        "Button"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {} ", tabs(indent), self.get_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
}