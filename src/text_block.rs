use std::collections::HashMap;

use crate::{events::Event, ui_elements::{get_attribute, tabs, UIAlloc, UICommon, UIElement, UIElementRef}, visitor::Visitor};

pub struct TextBlock {
    text: String,
    _font_weight: String,
    _font_size: String,
    _foreground: String,
    common: UICommon,
}


impl UIAlloc for TextBlock {
    fn new(attributes: &HashMap<String, String>, id: String) -> TextBlock {
        TextBlock {
            text: get_attribute(&attributes, "Text", ""),
            _font_weight: get_attribute(&attributes, "TextWeight", ""),
            _font_size: get_attribute(&attributes, "FontSize", ""),
            _foreground: get_attribute(&attributes, "Foreground", ""),
            common: UICommon::new(attributes, "TextBlock", id),
        }
    }
}

impl UIElement for TextBlock {
    fn get_id(&self) -> String {
        self.common.get_id()
    }
    fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        self.common.find_by_id(id)
    }

    fn handle_event(&self, ev: Event) {
        println!("NOTICE: textblock-handle-event");

        self.common.handle_event(ev);
    }


    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attr_opt(s)
    }
    
    fn get_ui_type_name(&self) -> &'static str {
        "TextBlock"
    }
    
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }

    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {} - content:{}",
            tabs(indent),
            self.get_ui_type_name(),
            self.text
        );
        self.common.dump(indent);
    }
    
    fn add_content_string(&mut self, s: String) {
        self.text = s
    }
    
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_text_block(self);
        self.common.visit(visitor);
        visitor.visit_text_block(self);
    }
}
