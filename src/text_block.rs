use crate::{ui_elements::{get_attribute, tabs, UICommon, UIElement, UIElementRef}, visitor::Visitor};

pub struct TextBlock {
    text: String,
    font_weight: String,
    font_size: String,
    foreground: String,
    common: UICommon,
}

impl TextBlock {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> TextBlock {
        TextBlock {
            text: get_attribute(&attributes, "Text", ""),
            font_weight: get_attribute(&attributes, "TextWeight", ""),
            font_size: get_attribute(&attributes, "FontSize", ""),
            foreground: get_attribute(&attributes, "Foreground", ""),
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for TextBlock {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    
    fn get_ui_type_name(&self) -> &'static str {
        "TextBlock"
    }
    
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
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
