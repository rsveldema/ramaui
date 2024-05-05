use std::collections::HashMap;

use crate::{
    events::Event,
    ui_elements::{get_attribute, tabs, UIAlloc, UICommon, UIElement, UIElementRef},
    visitor::Visitor,
};

pub struct GridLayout {
    common: UICommon,
    _name: String,
    _show_grid_lines: String,
    _background: String,
}

pub struct GridColumnDefinitions {
    common: UICommon,
}
pub struct GridRowDefinitions {
    common: UICommon,
}

pub struct ColumnDefinition {
    common: UICommon,
}

pub struct RowDefinition {
    common: UICommon,
}

impl UIAlloc for GridLayout {
    fn new(attributes: &HashMap<String, String>, id: String) -> GridLayout {
        GridLayout {
            _name: get_attribute(&attributes, "Name", ""),
            _background: get_attribute(&attributes, "Background", ""),
            _show_grid_lines: get_attribute(&attributes, "ShowGridLines", ""),
            common: UICommon::new(attributes, "Grid", id),
        }
    }
}

impl UIElement for GridLayout {
    fn get_id(&self) -> String {
        self.common.get_id()
    }    fn find_by_id(&self, id: String) -> Option<UIElementRef> {
        self.common.find_by_id(id)
    }


    fn handle_event(&self, ev: Event) {
        println!("NOTICE: grid-handle-event");
        self.common.handle_event(ev);
    }

    fn set_parent(&mut self, parent: UIElementRef) {
        self.common.set_parent(parent);
    }

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attr_opt(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Grid"
    }
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_ui_type_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_grid(self);
        self.common.visit(visitor);
        visitor.visit_grid(self);
    }
}

impl UIAlloc for GridColumnDefinitions {
    fn new(attributes: &HashMap<String, String>, id: String) -> GridColumnDefinitions {
        GridColumnDefinitions {
            common: UICommon::new(attributes, "GridColumnDefinitions", id),
        }
    }
}

impl UIElement for GridColumnDefinitions {
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

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attr_opt(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Grid_ColumnDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_ui_type_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_grid_cols(self);
        self.common.visit(visitor);
        visitor.visit_grid_cols(self);
    }
}

impl UIAlloc for GridRowDefinitions {
    fn new(attributes: &HashMap<String, String>, id: String) -> GridRowDefinitions {
        GridRowDefinitions {
            common: UICommon::new(attributes, "GridRowDefinitions", id),
        }
    }
}

impl UIElement for GridRowDefinitions {
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

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attr_opt(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Grid_RowDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_ui_type_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_grid_row(self);
        self.common.visit(visitor);
        visitor.visit_grid_row(self);
    }
}

impl UIAlloc for ColumnDefinition {
    fn new(attributes: &HashMap<String, String>, id: String) -> ColumnDefinition {
        ColumnDefinition {
            common: UICommon::new(attributes, "ColumnDefinition", id),
        }
    }
}
impl UIElement for ColumnDefinition {
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

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attr_opt(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "ColumnDefinition"
    }
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_ui_type_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_col_def(self);
        self.common.visit(visitor);
        visitor.visit_col_def(self);
    }
}

impl UIAlloc for RowDefinition {
    fn new(attributes: &HashMap<String, String>, id: String) -> RowDefinition {
        RowDefinition {
            common: UICommon::new(attributes, "RowDefinition", id),
        }
    }
}
impl UIElement for RowDefinition {
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

    fn get_attribute(&self, s: &str) -> Option<&String> {
        self.common.get_attr_opt(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "RowDefinition"
    }
    fn add_child(&mut self, child: UIElementRef, parent: UIElementRef) {
        self.common.add_child(child, parent);
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_ui_type_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}

    fn visit(&self, visitor: &mut dyn Visitor) {
        visitor.start_visit_row_def(self);
        self.common.visit(visitor);
        visitor.visit_row_def(self);
    }
}
