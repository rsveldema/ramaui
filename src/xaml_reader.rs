use std::fs::File;
use std::io::BufReader;
use parking_lot::Mutex;
use std::sync::Arc;
use xml::reader::{EventReader, XmlEvent};

use crate::{button::Button, content_page::ContentPage, grid_layout::{ColumnDefinition, GridColumnDefinitions, GridLayout, GridRowDefinitions, RowDefinition}, label::Label, stack_layout::StackLayout, text_block::TextBlock, ui_elements::{UIAlloc, UIElement, UIElementRef}, unknown_ui_elt::Unknown, window::Window};

fn static_leaker<T: UIElement + UIAlloc + 'static>(attributes: Vec<xml::attribute::OwnedAttribute>) -> UIElementRef
{
    let inner = T::new(attributes);
    let outer = Arc::new(Mutex::new(inner));
    return outer;
}

fn create_ui_element(name: &str, attributes: Vec<xml::attribute::OwnedAttribute>) -> UIElementRef {
    match name {
        "Label" => static_leaker::<Label>(attributes),
        "ContentPage" => static_leaker::<ContentPage>(attributes),
        "Button" =>  static_leaker::<Button>(attributes),
        "Window" => static_leaker::<Window>(attributes),
        "Grid" => static_leaker::<GridLayout>(attributes),
        "StackPanel" => static_leaker::<StackLayout>(attributes),
        "Grid.ColumnDefinitions" =>  static_leaker::<GridColumnDefinitions>(attributes),
        "Grid.RowDefinitions" => static_leaker::<GridRowDefinitions>(attributes),
        "ColumnDefinition" => static_leaker::<ColumnDefinition>(attributes),
        "RowDefinition" => static_leaker::<RowDefinition>(attributes),
        "TextBlock" => static_leaker::<TextBlock>(attributes),
        _ => static_leaker::<Unknown>(attributes),
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
                
                let last = parse_stack.last();
                if let Some(l) = last {
                    let mut k = l.lock();
                    k.add_child(new_elt.clone(), l.clone());
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
                let last = parse_stack.last();
                if let Some(l) = last {
                    let mut k = l.lock();
                    k.add_content_string(s);
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

    assert!(parse_stack.len() == 1);

    if let Some(last) = parse_stack.last()
    {
        return Result::Ok( last.clone() );
    }
    Result::Err(std::io::Error::new(std::io::ErrorKind::NotFound, "unknown parse stack problem"))
}



#[cfg(test)]
mod tests {
    use crate::xaml_reader;

    #[test]
    fn it_works() {
        let tree = xaml_reader::read_xaml(&"tests/labeltest.xaml".to_string());        
        if let Result::Ok(t) = tree {
            println!("TREE ---> ");
            let tree = t.lock();
            assert!(tree.get_ui_type_name() == "Window");
            tree.dump(0)
            
        } else {
            println!("no tree returned from xaml parse?");
            assert!(false);
        }
    }
}
