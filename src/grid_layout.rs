use crate::{ui_elements::{get_attribute, tabs, UICommon, UIElement, UIElementRef}, visitor::Visitor};

pub struct GridLayout {
    common: UICommon,
    name: String,
    show_grid_lines: String,
    background: String,
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

impl GridLayout {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> GridLayout {
        GridLayout {
            name: get_attribute(&attributes, "Name", ""),
            background: get_attribute(&attributes, "Background", ""),
            show_grid_lines: get_attribute(&attributes, "ShowGridLines", ""),
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for GridLayout {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Grid"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
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

impl GridColumnDefinitions {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> GridColumnDefinitions {
        GridColumnDefinitions {
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for GridColumnDefinitions {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Grid_ColumnDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
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

impl GridRowDefinitions {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> GridRowDefinitions {
        GridRowDefinitions {
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for GridRowDefinitions {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "Grid_RowDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
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

impl ColumnDefinition {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> ColumnDefinition {
        ColumnDefinition {
            common: UICommon::new(attributes),
        }
    }
}
impl UIElement for ColumnDefinition {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "ColumnDefinition"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
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

impl RowDefinition {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> RowDefinition {
        RowDefinition {
            common: UICommon::new(attributes),
        }
    }
}
impl UIElement for RowDefinition {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_ui_type_name(&self) -> &'static str {
        "RowDefinition"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
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
