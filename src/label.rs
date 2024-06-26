use std::collections::HashMap;

use crate::{events::Event, ui_elements::{tabs, UIAlloc, UICommon, UIElement, UIElementRef}, visitor::Visitor};

pub struct Label {
    common: UICommon,
}

impl Label {    
    pub fn get_content(&self) -> String { self.common.get_attr("Text") } 
}


impl UIAlloc for Label {
    fn new(attributes: &HashMap<String, String>, id: String) -> Label {
        Label {
            common: UICommon::new(attributes,  "Label", id),
        }
    }
}


impl UIElement for Label {
    fn get_id(&self) -> String {
        self.common.get_id()
    }
    fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        self.common.find_by_id(id)
    }


    fn handle_event(&self, ev: Event) {
        println!("NOTICE: label-handle-event");
        self.common.handle_event(ev);
    }


    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attr_opt(s)
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
            self.common.get_attr("Text")
        );
        self.common.dump(indent);
    }

    fn add_content_string(&mut self, s: String) {
        self.common.set_attr("Text", s)
    }
    
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_label(self);
        self.common.visit(visitor);
        visitor.visit_label(self);
    }
}