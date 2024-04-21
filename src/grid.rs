use crate::ui_elements::{tabs, UICommon, UIElement, UIElementRef, get_attribute};

pub struct Grid {
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

impl Grid {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Grid {
        Grid {
            name: get_attribute(&attributes, "Name", ""),
            background: get_attribute(&attributes, "Background", ""),
            show_grid_lines: get_attribute(&attributes, "ShowGridLines", ""),
            common: UICommon::new(attributes),
        }
    }
}

impl UIElement for Grid {
    fn get_attribute(&self, s: &String) -> Option<String> {
        self.common.get_attribute(s)
    }
    fn get_name(&self) -> &'static str {
        "Grid"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
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
    fn get_name(&self) -> &'static str {
        "Grid_ColumnDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
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
    fn get_name(&self) -> &'static str {
        "Grid_RowDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
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
    fn get_name(&self) -> &'static str {
        "ColumnDefinition"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
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
    fn get_name(&self) -> &'static str {
        "RowDefinition"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.common.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.common.dump(indent);
    }
    fn add_content_string(&mut self, _: String) {}
}
