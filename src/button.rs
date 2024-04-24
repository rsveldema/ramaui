use crate::{ui_elements::{tabs, UICommon, UIElement, UIElementRef}, visitor::Visitor};


pub struct Button {
    common: UICommon,
}

impl Button {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Button {
        Button {
            common: UICommon::new(attributes),
        }
    }
    pub fn get_text(&self) -> String { 
        if let Some(s) = self.common.get_attribute(&"Content".to_string()) {
            println!("content of button = {}", s);
            return s
        }
        println!("no content string in button");
        return "".to_string();
    }
}

impl UIElement for Button {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Button"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {} ", tabs(indent), self.get_ui_type_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_button(self);
        self.common.visit(visitor);
        visitor.visit_button(self);
    }
}
