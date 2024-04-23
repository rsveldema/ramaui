use std::env;

mod button;
mod content_page;
mod grid;
mod label;
mod text_block;
mod ui_elements;
mod unknown_ui_elt;
mod visitor;
mod window;
mod xaml_reader;

use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use ui_elements::{UIElement, UIElementRef};
use visitor::Visitor;

use crate::{
    button::Button,
    content_page::ContentPage,
    grid::{ColumnDefinition, Grid, GridColumnDefinitions, GridRowDefinitions, RowDefinition},
    label::Label,
    text_block::TextBlock,
    unknown_ui_elt::Unknown,
    window::Window,
};

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

struct UIBuilder<'b> {
    root: Option<ApplicationWindow>,
    app: &'b Application,
}

impl<'b> UIBuilder<'b> {
    fn new(app: &'b Application) -> UIBuilder<'b> {
        return UIBuilder { root: Option::None, app: app};
    }
}

impl<'lifetime> Visitor for UIBuilder<'lifetime> {
    fn visit_button(&mut self, b: &Button) {}
    fn visit_window(&mut self, w: &Window) {
        if self.root.is_none() {
            self.root = Option::Some(ApplicationWindow::builder()
                .application(self.app)
                .title(w.get_title())
                .default_width(w.get_width())
                .default_height(w.get_height())
                .build());
        }
    }
    fn visit_label(&mut self, l: &Label) {}
    fn visit_text_block(&mut self, t: &TextBlock) {}
    fn visit_grid(&mut self, g: &Grid) {}
    fn visit_grid_cols(&mut self, g: &GridColumnDefinitions) {}
    fn visit_grid_row(&mut self, g: &GridRowDefinitions) {}
    fn visit_col_def(&mut self, g: &ColumnDefinition) {}
    fn visit_row_def(&mut self, g: &RowDefinition) {}
    fn visit_content_page(&mut self, g: &ContentPage) {}
    fn visit_unknown(&mut self, g: &Unknown) {}
}

fn build_ui_from_xaml(app: &Application, root: UIElementRef) -> Option<ApplicationWindow> {
    let mut builder = UIBuilder::new(app);
    root.as_ref().lock().unwrap().visit(&mut builder);
    return builder.root;
}

fn open_ui(app: &Application, files: &[gtk::gio::File], _s: &str, root: UIElementRef) {
    for file in files {
        println!("Open--: {:?}", file.path());
    }

    let window_opt = build_ui_from_xaml(app, root);
    if let Some(window) = window_opt {
        window.present();
    } else {
        panic!("failed to build ui from XAML");
    }
}

fn start_interpreter(root: &UIElementRef) -> glib::ExitCode {
    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    // clone to get rid of the borrow of the parameter
    let k = root.clone();
    app.connect_open(
        move |app: &Application, files: &[gtk::gio::File], s: &str| {
            // clone k to pass into open-ui
            open_ui(app, files, s, k.clone());
        },
    );

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
