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
    fn dump(&self);
}

pub struct UIChildren {
    children: Vec<UIElementRef>,
}

impl UIChildren {
    fn add_child(&mut self, child: UIElementRef) {
        self.children.push(child)
    }

    fn dump(&self) {
        for c in self.children.iter() {
            let l = c.lock();
            l.unwrap().dump();
        }
    }
}

pub struct ContentPage {
    title: String,
    children: UIChildren,
}

pub struct Label {
    content: String,
    children: UIChildren,
}

pub struct Button {
    children: UIChildren
}

impl UIElement for Label {
    fn get_name(&self) -> &'static str {
        "Label"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self) {
        println!("DUMP: {} - content:{}", self.get_name(), self.content);
        self.children.dump();
    }
}

impl UIElement for ContentPage {
    fn get_name(&self) -> &'static str {
        "ContentPage"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self) {
        println!("DUMP: {}  -  title:{}", self.get_name(), self.title);
        self.children.dump();
    }
}

impl UIElement for Button {
    fn get_name(&self) -> &'static str {
        "Button"
    }
    fn add_child(&mut self, child: UIElementRef) {
        self.children.add_child(child)
    }
    fn dump(&self) {
        println!("DUMP: {} ", self.get_name());
        self.children.dump();
    }
}

fn get_attribute(
    attributes: Vec<xml::attribute::OwnedAttribute>,
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
            content: get_attribute(attributes, "Text", ""),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "ContentPage" => Rc::new(Mutex::new(ContentPage {
            title: get_attribute(attributes, "Title", "Title"),
            children: UIChildren {
                children: Vec::new(),
            },
        })),
        "Button" => Rc::new(Mutex::new(Button {
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
            }
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
        t.lock().unwrap().dump()
    } else {
        println!("no tree returned from xaml parse?");
    }
}
