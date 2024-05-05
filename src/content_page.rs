use crate::{events::Event, ui_elements::{get_attribute, tabs, UIAlloc, UICommon, UIElement, UIElementRef}, visitor::Visitor};


pub struct ContentPage {
    title: String,
    common: UICommon,
}


impl UIAlloc for ContentPage {
    fn new(attributes: Vec<xml::attribute::OwnedAttribute>, id: String) -> ContentPage {
        ContentPage {
            title: get_attribute(&attributes, "Title", "Title"),
            common: UICommon::new(attributes, id),
        }
    }
}

impl UIElement for ContentPage {
    fn get_id(&self) -> String {
        self.common.get_id()
    }
    fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        self.common.find_by_id(id)
    }

    fn handle_event(&self, ev: Event) {
        self.common.handle_event(ev);
    }


    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }

    fn get_attribute(&self, s: &str) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "ContentPage"
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
        visitor.start_visit_content_page(self);
        self.common.visit(visitor);
        visitor.visit_content_page(self);
    }
}