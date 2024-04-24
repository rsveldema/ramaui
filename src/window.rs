use crate::{ui_elements::{get_attribute, tabs, UICommon, UIElement, UIElementRef}, visitor::Visitor};

pub struct Window {
    title: String,
    window_style: String,
    common: UICommon,
}

impl Window {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Window {
        let mut w = Window {
            title: get_attribute(&attributes, "Title", "Title"),
            window_style: get_attribute(&attributes, "WindowStyle", ""),
            common: UICommon::new(attributes),
        };
        
        if w.common.get_width().is_none() {
            w.set_width(100);
        }

        if w.common.get_height().is_none() {
            w.set_height(100);
        }
        w
    }

    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_width(&self) -> i32 { self.common.get_width().unwrap() }
    pub fn get_height(&self) -> i32 { self.common.get_height().unwrap() }
    pub fn set_width(&mut self, width: i32) { self.common.set_width(width); }
    pub fn set_height(&mut self, height: i32) { self.common.set_height(height); }
}

impl UIElement for Window {

    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Window"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
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
