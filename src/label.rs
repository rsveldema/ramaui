use crate::ui_elements::{get_attribute, tabs, UICommon, UIElement, UIElementRef};

pub struct Label {
    content: String,
    common: UICommon,
}

impl Label {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Label {
        Label {
            content: get_attribute(&attributes, "Text", ""),
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for Label {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_name(&self) -> &'static str {
        "Label"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {} - content:{}",
            tabs(indent),
            self.get_name(),
            self.content
        );
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {
        self.content = s;
    }
}