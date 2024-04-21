use crate::ui_elements::{get_attribute, tabs, UICommon, UIElement, UIElementRef};

pub struct Window {
    title: String,
    window_style: String,
    common: UICommon,
}

impl Window {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Window {
        Window {
            title: get_attribute(&attributes, "Title", "Title"),
            window_style: get_attribute(&attributes, "WindowStyle", ""),
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for Window {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_name(&self) -> &'static str {
        "Window"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {}  -  title:{}",
            tabs(indent),
            self.get_name(),
            self.title
        );
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
}
