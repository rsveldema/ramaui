use std::collections::HashMap;

use crate::{events::Event, ui_elements::{get_attribute, tabs, UIAlloc, UICommon, UIElement, UIElementRef}, visitor::Visitor};


pub struct Button {
    content: String,
    common: UICommon,
}

impl UIAlloc for Button {
    fn new(attributes: &HashMap<String, String>, id: String) -> Button {
        Button {
            content: get_attribute(&attributes, "Content", ""),
            common: UICommon::new(attributes, id),
        }
    }
}

impl Button {
    pub fn get_text(&self) -> String { 
        self.content.to_string()
    }
}

impl UIElement for Button {
    fn get_id(&self) -> String {
        self.common.get_id()
    }

    fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        self.common.find_by_id(id)
    }

    fn handle_event(&self, ev: Event) {
        println!("NOTICE: button-handle-event");
        self.common.handle_event(ev);
    }

    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attribute(s)
    }
    
    fn get_ui_type_name(&self) -> &'static str {
        "Button"
    }
    
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {} ", tabs(indent), self.get_ui_type_name());
        self.common.dump(indent);
    }

    fn add_content_string(&mut self, s: String) {
        self.content = s;
    }

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_button(self);
        self.common.visit(visitor);
        visitor.visit_button(self);
    }
}
