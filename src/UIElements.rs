use std::{rc::Rc, sync::Mutex};

pub type UIElementRef = Rc<Mutex<dyn UIElement>>;

fn get_attribute(
    attributes: &Vec<xml::attribute::OwnedAttribute>,
    name: &str,
    default_str: &str,
) -> String {
    for a in attributes.iter() {
        if a.name.local_name == name {
            return a.value.to_string();
        }
    }
    default_str.to_string()
}

pub trait UIElement {
    fn get_name(&self) -> &'static str;
    fn add_child(&mut self, child: UIElementRef);
    fn dump(&self, indent: i32);
    fn add_content_string(&mut self, s: String);
}

pub struct UIChildren {
    children: Vec<UIElementRef>,
}

impl UIChildren {
    fn add_child(&mut self, child: UIElementRef) {
        self.children.push(child)
    }

    fn dump(&self, _indent: i32) {
        let indent = 1 + _indent;
        for c in self.children.iter() {
            let l = c.lock();
            l.unwrap().dump(indent);
        }
    }
}

pub struct ContentPage {
    title: String,
    children: UIChildren,
}

pub struct Window {
    title: String,
    width: String,
    height: String,
    window_style: String,
    children: UIChildren,
}

pub struct Label {
    content: String,
    children: UIChildren,
}

pub struct Grid {
    children: UIChildren,
    name: String,
    width: String,
    height: String,
    show_grid_lines: String,
    background: String,
}

pub struct Grid_ColumnDefinitions {
    children: UIChildren,
}
pub struct Grid_RowDefinitions {
    children: UIChildren,
}

pub struct ColumnDefinition {
    children: UIChildren,
    width: String,
    height: String,
}

pub struct RowDefinition {
    children: UIChildren,
    width: String,
    height: String,
}

pub struct TextBlock {
    text: String,
    font_weight: String,
    font_size: String,
    width: String,
    height: String,
    foreground: String,
    children: UIChildren,
}

pub struct Button {
    children: UIChildren,
}

fn tabs(c: i32) -> String {
    let mut k: String = String::new();
    for i in 0..c {
        k.push_str("   ");
    }
    k
}

impl Grid {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Grid {
        Grid {
            name: get_attribute(&attributes, "Name", ""),
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
            background: get_attribute(&attributes, "Background", ""),
            show_grid_lines: get_attribute(&attributes, "ShowGridLines", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}

impl UIElement for Grid {
    fn get_name(&self) -> &'static str {
        "Grid"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}


impl Grid_ColumnDefinitions {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Grid_ColumnDefinitions {
        Grid_ColumnDefinitions {
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}


impl UIElement for Grid_ColumnDefinitions {
    fn get_name(&self) -> &'static str {
        "Grid_ColumnDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}


impl Grid_RowDefinitions {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Grid_RowDefinitions {
        Grid_RowDefinitions {
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}

impl UIElement for Grid_RowDefinitions {
    fn get_name(&self) -> &'static str {
        "Grid_RowDefinitions"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}

impl ColumnDefinition {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> ColumnDefinition {
        ColumnDefinition {
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}
impl UIElement for ColumnDefinition {
    fn get_name(&self) -> &'static str {
        "ColumnDefinition"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}

impl UIElement for RowDefinition {
    fn get_name(&self) -> &'static str {
        "RowDefinition"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {}", tabs(indent), self.get_name());
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}


impl TextBlock {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> TextBlock {
        TextBlock {
            text: get_attribute(&attributes, "Text", ""),
            font_weight: get_attribute(&attributes, "TextWeight", ""),
            font_size: get_attribute(&attributes, "FontSize", ""),
            height: get_attribute(&attributes, "Height", ""),
            width: get_attribute(&attributes, "Width", ""),
            foreground: get_attribute(&attributes, "Foreground", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}

impl UIElement for TextBlock {
    fn get_name(&self) -> &'static str {
        "TextBlock"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {} - content:{}",
            tabs(indent),
            self.get_name(),
            self.text
        );
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {
        self.text = s
    }
}


impl Label {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Label {
        Label {
            content: get_attribute(&attributes, "Text", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}

impl UIElement for Label {
    fn get_name(&self) -> &'static str {
        "Label"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {} - content:{}",
            tabs(indent),
            self.get_name(),
            self.content
        );
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}


impl ContentPage {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> ContentPage {
        ContentPage {
            title: get_attribute(&attributes, "Title", "Title"),
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}

impl UIElement for ContentPage {
    fn get_name(&self) -> &'static str {
        "ContentPage"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {}  -  title:{}",
            tabs(indent),
            self.get_name(),
            self.title
        );
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}


impl Window {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Window {
        Window {
            title: get_attribute(&attributes, "Title", "Title"),
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
            window_style: get_attribute(&attributes, "WindowStyle", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}


impl UIElement for Window {
    fn get_name(&self) -> &'static str {
        "Window"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!(
            "{}DUMP: {}  -  title:{}",
            tabs(indent),
            self.get_name(),
            self.title
        );
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}



impl Button {
    pub fn new(attributes: Vec<xml::attribute::OwnedAttribute>) -> Button {
        Button {
            children: UIChildren {
                children: Vec::new(),
            },
        }
    }
}


impl UIElement for Button {
    fn get_name(&self) -> &'static str {
        "Button"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self, indent: i32) {
        println!("{}DUMP: {} ", tabs(indent), self.get_name());
        self.children.dump(indent);
    }
    fn add_content_string(&mut self, s: String) {}
}
