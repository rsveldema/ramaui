use crate::{events::Event, ui_elements::{get_attribute, tabs, UIAlloc, UICommon, UIElement, UIElementRef}, visitor::Visitor};

pub struct Label {
    content: String,
    common: UICommon,
}

impl Label {    
    pub fn get_content(&self) -> &String { &self.content } 
}


impl UIAlloc for Label {
    fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Label {
        Label {
            content: get_attribute(&attributes, "Text", ""),
            common: UICommon::new(attributes),
        }
    }
}


impl UIElement for Label {
    fn handle_event(&self, ev: Event) {
        self.common.handle_event(ev);
    }


    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }

    fn get_attribute(&self, s: &str) -> Option<String> {
        self.common.get_attribute(s)
    }
    
    fn get_ui_type_name(&self) -> &'static str {
        "Label"
    }
    
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {} - content:{}",
            tabs(indent),
            self.get_ui_type_name(),
            self.content
        );
        self.common.dump(indent);
    }

    fn add_content_string(&mut self, s: String) {
        self.content = s;
    }
    
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_label(self);
        self.common.visit(visitor);
        visitor.visit_label(self);
    }
}