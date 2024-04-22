use std::env;

mod ui_elements;
mod xaml_reader;
mod button;
mod label;
mod text_block;
mod content_page;
mod window;
mod grid;
mod unknown_ui_elt;

use gtk::gio::ApplicationFlags;
use gtk::glib::ExitCode;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use ui_elements::UIElement;


const APP_ID: &str = "org.gtk_rs.HelloWorld1";


fn build_ui(app: &Application) {
    println!("build-ui");
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(320)
        .default_height(200)
        .build();

    // Present window
    window.present();
}

fn open_ui(app: &Application, files:&[gtk::gio::File], s: &str) {
    for file in files {
        println!("Open--: {:?}", file.path());
    } 
    
    println!("open_ui");
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(320)
        .default_height(200)
        .build();

    // Present window
    window.present();
}

fn start_interpreter(root: &dyn UIElement) -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).flags(ApplicationFlags::HANDLES_OPEN).build();

    app.connect_open(open_ui);

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
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

    let tree = xaml_reader::read_xaml(filename);
    if let Result::Ok(t) = tree {
        println!("TREE ---> ");
        let root = t.lock().unwrap();
        root.dump(0);
        start_interpreter(root);
    } else {
        println!("no tree returned from xaml parse?");
    }
}