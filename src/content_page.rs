use crate::{ui_elements::{get_attribute, tabs, UICommon, UIElement, UIElementRef}, visitor::Visitor};


pub struct ContentPage {
    title: String,
    common: UICommon,
}

impl ContentPage {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> ContentPage {
        ContentPage {
            title: get_attribute(&attributes, "Title", "Title"),
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for ContentPage {
    fn get_attribute(&self, s: &str) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "ContentPage"
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
        visitor.start_visit_content_page(self);
        self.common.visit(visitor);
        visitor.visit_content_page(self);
    }
}