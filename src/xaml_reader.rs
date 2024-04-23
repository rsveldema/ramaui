use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use std::sync::Mutex;
use xml::reader::{EventReader, XmlEvent};

use crate::{button::Button, content_page::ContentPage, grid::{ColumnDefinition, Grid, GridColumnDefinitions, GridRowDefinitions, RowDefinition}, label::Label, text_block::TextBlock, ui_elements::UIElementRef, unknown_ui_elt::Unknown, window::Window};


fn create_ui_element(name: &str, attributes: Vec<xml::attribute::OwnedAttribute>) -> UIElementRef {
    match name {
        "Label" => Rc::new(Mutex::new(Label::new(attributes))),
        "ContentPage" => Rc::new(Mutex::new(ContentPage::new(attributes))),
        "Button" => Rc::new(Mutex::new(Button::new(attributes))),
        "Window" => Rc::new(Mutex::new(Window::new(attributes))),
        "Grid" => Rc::new(Mutex::new(Grid::new(attributes))),
        "Grid.ColumnDefinitions" => Rc::new(Mutex::new(GridColumnDefinitions::new(attributes))),
        "Grid.RowDefinitions" => Rc::new(Mutex::new(GridRowDefinitions::new(attributes))),
        "ColumnDefinition" => Rc::new(Mutex::new(ColumnDefinition::new(attributes))),
        "RowDefinition" => Rc::new(Mutex::new(RowDefinition::new(attributes))),
        "TextBlock" => Rc::new(Mutex::new(TextBlock::new(attributes))),
        _ => Rc::new(Mutex::new(Unknown::new(attributes))),
    }
}

pub fn read_xaml(filename: &String) -> Result<UIElementRef, std::io::Error> {
    println!("Parsing: {}", filename);

    let file = File::open(filename)?;
    let file_reader = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file_reader);
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
            Ok(XmlEvent::Characters(s)) => {
                if !parse_stack.is_empty() {
                    let last = parse_stack.last();
                    last.unwrap().lock().unwrap().add_content_string(s);
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



#[cfg(test)]
mod tests {
    use crate::xaml_reader;

    #[test]
    fn it_works() {
        let tree = xaml_reader::read_xaml(&"tests/labeltest.xaml".to_string());        
        if let Result::Ok(t) = tree {
            println!("TREE ---> ");
            let tree = t.lock().unwrap();
            assert!(tree.get_ui_type_name() == "Window");
            tree.dump(0)
            
        } else {
            println!("no tree returned from xaml parse?");
            assert!(false);
        }
    }
}
