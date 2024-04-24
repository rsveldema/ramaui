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

use ui_builder::start_interpreter;


fn usage() {
    let args: Vec<String> = env::args().collect();
    println!("USAGE: <xaml>");
    for s in args {
        println!("saw arg: {}", s);
    }
    std::process::exit(1);
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
            start_interpreter(&t);
        }
        Result::Err(err) => {
            println!("failed to read xml: {}", err)
        }
    }
}
