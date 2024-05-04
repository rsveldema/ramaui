use crate::{ui_elements::{tabs, UICommon, UIElement, UIElementRef}, visitor::Visitor};


pub struct StackLayout {
    common: UICommon,
}

impl StackLayout {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> StackLayout {
        StackLayout {
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for StackLayout {
    fn get_attribute(&self, s: &str) -> Option<String> {
        self.common.get_attribute(s)
    }
    
    fn get_ui_type_name(&self) -> &'static str {
        "StackLayout"
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
        visitor.start_visit_stack(self);
        self.common.visit(visitor);
        visitor.visit_stack(self);
    }
}
