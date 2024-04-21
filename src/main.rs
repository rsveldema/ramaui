use std::env;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use std::sync::Mutex;
use xml::reader::{EventReader, XmlEvent};

type UIElementRef = Rc<Mutex<dyn UIElement>>;

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

fn create_ui_element(name: &str, attributes: Vec<xml::attribute::OwnedAttribute>) -> UIElementRef {
    match name {
        "Label" => Rc::new(Mutex::new(Label {
            content: get_attribute(&attributes, "Text", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "ContentPage" => Rc::new(Mutex::new(ContentPage {
            title: get_attribute(&attributes, "Title", "Title"),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "Button" => Rc::new(Mutex::new(Button {
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "Window" => Rc::new(Mutex::new(Window {
            title: get_attribute(&attributes, "Title", "Title"),
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
            window_style: get_attribute(&attributes, "WindowStyle", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "Grid" => Rc::new(Mutex::new(Grid {
            name: get_attribute(&attributes, "Name", ""),
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
            background: get_attribute(&attributes, "Background", ""),
            show_grid_lines: get_attribute(&attributes, "ShowGridLines", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "Grid.ColumnDefinitions" => Rc::new(Mutex::new(Grid_ColumnDefinitions {
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "Grid.RowDefinitions" => Rc::new(Mutex::new(Grid_RowDefinitions {
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "ColumnDefinition" => Rc::new(Mutex::new(ColumnDefinition {
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "RowDefinition" => Rc::new(Mutex::new(RowDefinition {
            width: get_attribute(&attributes, "Width", ""),
            height: get_attribute(&attributes, "Height", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "TextBlock" => Rc::new(Mutex::new(TextBlock {
            text: get_attribute(&attributes, "Text", ""),
            font_weight: get_attribute(&attributes, "TextWeight", ""),
            font_size: get_attribute(&attributes, "FontSize", ""),
            height: get_attribute(&attributes, "Height", ""),
            width: get_attribute(&attributes, "Width", ""),
            foreground: get_attribute(&attributes, "Foreground", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        _ => Rc::new(Mutex::new(Label {
            content: format!("<UNKNOWN: {name}"),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
    }
}

fn read_xaml(filename: &String) -> Result<UIElementRef, std::io::Error> {
    println!("Parsing: {}", filename);

    let file = File::open(filename)?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;

    let mut parse_stack: Vec<UIElementRef> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;

                let new_elt = create_ui_element(&name.local_name, attributes);
                if !parse_stack.is_empty() {
                    if let Some(last) = parse_stack.last_mut() {
                        last.lock().unwrap().add_child(new_elt.clone())
                    }
                }
                parse_stack.push(new_elt);
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);

                if parse_stack.len() > 1 {
                    parse_stack.pop();
                }
            },
            Ok(XmlEvent::Characters(s)) => {
                if parse_stack.len() > 0 {
                    let last = parse_stack.last();
                    last.unwrap().lock().unwrap().add_content_string(s);
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    let last = parse_stack.last();
    Ok(last.unwrap().clone())
}

fn usage() {
    let args: Vec<String> = env::args().collect();
    println!("USAGE: <xaml>");
    for s in args {
        println!("arg: {}", s);
    }
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let filename = &args[1];

    let tree = read_xaml(filename);
    if let Result::Ok(t) = tree {
        println!("TREE ---> ");
        t.lock().unwrap().dump(0)
    } else {
        println!("no tree returned from xaml parse?");
    }
}
