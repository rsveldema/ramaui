use std::collections::HashMap;

use crate::{events::Event, ui_elements::{tabs, UIAlloc, UICommon, UIElement, UIElementRef}, visitor::Visitor};


pub struct StackLayout {
    common: UICommon,
}


impl UIAlloc for StackLayout {
    fn new(attributes: &HashMap<String, String>, id: String) -> StackLayout {
        StackLayout {
            common: UICommon::new(attributes, id),
        }
    }
}

impl UIElement for StackLayout {
    fn get_id(&self) -> String {
        self.common.get_id()
    }
    fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        self.common.find_by_id(id)
    }

    fn handle_event(&self, ev: Event) {
        println!("NOTICE: stacklayout-handle-event");

        self.common.handle_event(ev);
    }


    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attribute(s)
    }
    
    fn get_ui_type_name(&self) -> &'static str {
        "StackLayout"
    }
    
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent)
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
