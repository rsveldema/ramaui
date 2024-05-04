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

use ramaui::inspectable;
use ui_builder::start_interpreter;


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


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
    }

    let filename = &args[1];

    let tree = xaml_reader::read_xaml(filename);
    match tree {
        Result::Ok(t) => {
            t.as_ref().lock().unwrap().dump(0);

            let win:&'static mut MainWindow = Box::leak::<'static>(Box::new(MainWindow::new()));

            let cwin: &'static mut dyn CallableByName = win as &'static mut dyn CallableByName;
            let w: std::rc::Rc<std::sync::Mutex<&'static dyn CallableByName>>  = std::rc::Rc::new(std::sync::Mutex::new(cwin));

            start_interpreter(&t, w);
        }
        Result::Err(err) => {
            println!("failed to read xml: {}", err)
        }
    }
}
