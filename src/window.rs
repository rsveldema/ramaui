use std::collections::HashMap;

use crate::{events::Event, ui_elements::{get_attribute, tabs, UIAlloc, UICommon, UIElement, UIElementRef}, visitor::Visitor};

pub struct Window {
    title: String,
    _window_style: String,
    common: UICommon,
}


impl UIAlloc for Window {
    fn new(attributes: &HashMap<String, String>, id: String) -> Window {
        let mut w = Window {
            title: get_attribute(&attributes, "Window.Title", "Title"),
            _window_style: get_attribute(&attributes, "Window.WindowStyle", ""),
            common: UICommon::new(&attributes, "Window", id),
        };
        
        if w.common.get_width().is_none() {
            w.set_width(100);
        }

        if w.common.get_height().is_none() {
            w.set_height(100);
        }
        w
    }
}

impl Window {
    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_width(&self) -> i32 { self.common.get_width().unwrap() }
    pub fn get_height(&self) -> i32 { self.common.get_height().unwrap() }
    pub fn set_width(&mut self, width: i32) { self.common.set_width(width); }
    pub fn set_height(&mut self, height: i32) { self.common.set_height(height); }
}

impl UIElement for Window {
    fn get_id(&self) -> String {
        self.common.get_id()
    }
    fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        self.common.find_by_id(id)
    }

    fn handle_event(&self, ev: Event) {
        println!("NOTICE: window-handle-event");
        self.common.handle_event(ev);
    }


    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }


    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attribute(&s.to_string())
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Window"
    }
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {}  -  title:{}",
            tabs(indent),
            self.get_ui_type_name(),
            self.title
        );
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
    
    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_window(self);
        self.common.visit(visitor);
        visitor.visit_window(self);
    }
}
