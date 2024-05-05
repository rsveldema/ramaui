use std::env;

mod button;
mod content_page;
mod grid_layout;
mod label;
mod stack_layout;
mod text_block;
mod ui_elements;
mod unknown_ui_elt;
mod visitor;
mod window;
mod xaml_reader;
mod ui_builder;
mod callable;
mod events;
use parking_lot::Mutex;
use ramaui::inspectable;
use ui_builder::start_interpreter;
use ui_elements::{UIElementRef, UITree, UITreeRef};


fn usage() {
    let args: Vec<String> = env::args().collect();
    println!("USAGE: <xaml>");
    for s in args {
        println!("saw arg: {}", s);
    }
    std::process::exit(1);
}



#[inspectable(bar)]
impl MainWindow {
    pub fn button_click(&self) {
        println!("called MainWindow::button click!");
    }
    pub fn checkbox_click(&self) {
        println!("called MainWindow::checkbox click!");
    }
}

fn create_tree(root_elt_ref: UIElementRef) -> Option<UITreeRef> {
    let mut tree = UITree::new();
    tree.root = Some(root_elt_ref.clone());
    let r = Box::leak(Box::new(tree));
    return Some(r);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let filename = &args[1];

    let root_elt_ref = xaml_reader::read_xaml(filename);
    match root_elt_ref {
        Result::Ok(t) => {
            t.lock().dump(0);

            let win = Box::leak::<'static>(Box::new(Mutex::new(MainWindow::new())));

            {
                let mut r = win.lock();
                let tree = create_tree(t);
                r.set_tree(tree);
            }

            start_interpreter(win);
        }
        Result::Err(err) => {
            println!("failed to read xml: {}", err)
        }
    }
}
