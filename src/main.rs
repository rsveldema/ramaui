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

use gtk::builders::{ApplicationBuilder, ButtonBuilder};
use gtk::ffi::GtkButton;
use gtk::gio::ApplicationFlags;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use stack_layout::StackLayout;
use ui_elements::UIElementRef;
use visitor::Visitor;

use crate::{
    button::Button,
    content_page::ContentPage,
    grid_layout::{
        ColumnDefinition, GridColumnDefinitions, GridLayout, GridRowDefinitions, RowDefinition,
    },
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

enum GtkPushed {
    Button(gtk::Button),
    Label(gtk::Label),
}

struct UIBuilder<'b> {
    root: Option<ApplicationWindow>,
    app: &'b Application,
    nested_gtk_items: Vec<Vec<GtkPushed>>,
}

impl<'b> UIBuilder<'b> {
    fn new(app: &'b Application) -> UIBuilder<'b> {
        return UIBuilder {
            root: Option::None,
            app: app,
            nested_gtk_items: Vec::new(),
        };
    }

    fn enter_scope(&mut self) {
        self.nested_gtk_items.push(Vec::new());
    }

    fn leave_scope(&mut self) {
        self.nested_gtk_items.pop();
    }

    fn last_scope(&mut self) -> &mut Vec<GtkPushed> {
        return self.nested_gtk_items.last_mut().unwrap();
    }
}

impl<'lifetime> Visitor for UIBuilder<'lifetime> {
    fn start_visit_button(&mut self, b: &Button) {
        self.enter_scope()
    }
    fn start_visit_window(&mut self, w: &Window) {
        self.enter_scope()
    }
    fn start_visit_label(&mut self, l: &Label) {
        self.enter_scope()
    }
    fn start_visit_text_block(&mut self, t: &TextBlock) {
        self.enter_scope()
    }
    fn start_visit_grid(&mut self, g: &GridLayout) {
        self.enter_scope()
    }
    fn start_visit_grid_cols(&mut self, g: &GridColumnDefinitions) {
        self.enter_scope()
    }
    fn start_visit_grid_row(&mut self, g: &GridRowDefinitions) {
        self.enter_scope()
    }
    fn start_visit_col_def(&mut self, g: &ColumnDefinition) {
        self.enter_scope()
    }
    fn start_visit_row_def(&mut self, g: &RowDefinition) {
        self.enter_scope()
    }
    fn start_visit_content_page(&mut self, g: &ContentPage) {
        self.enter_scope()
    }
    fn start_visit_unknown(&mut self, g: &Unknown) {
        self.enter_scope()
    }
    fn start_visit_stack(&mut self, g: &StackLayout) {
        self.enter_scope()
    }

    fn visit_button(&mut self, b: &Button) {
        let b = gtk::Button::with_label(&b.get_text());
        self.leave_scope();
        self.last_scope().push(GtkPushed::Button(b));
    }

    fn visit_window(&mut self, w: &Window) {
        let win = ApplicationWindow::builder()
            .application(self.app)
            .title(w.get_title())
            .default_width(w.get_width())
            .default_height(w.get_height())
            .build();

        for elt in self.last_scope() {
            match elt {
                GtkPushed::Label(l) => {
                    println!("adding label to window");
                    win.set_child(Some(l));
                }
                GtkPushed::Button(l) => {
                    println!("adding button to window");
                    win.set_child(Some(l));
                }
            }
        }

        if self.root.is_none() {
            self.root = Option::Some(win);
        }
        self.leave_scope();
    }
    fn visit_label(&mut self, l: &Label) {
        let b = gtk::Label::new(Option::Some(l.get_content().as_str()));

        self.leave_scope();
        self.last_scope().push(GtkPushed::Label(b));
    }
    fn visit_text_block(&mut self, t: &TextBlock) {
        self.leave_scope();
    }
    fn visit_grid(&mut self, g: &GridLayout) {
        self.leave_scope();
    }
    fn visit_stack(&mut self, g: &StackLayout) {
        self.leave_scope();
    }
    fn visit_grid_cols(&mut self, g: &GridColumnDefinitions) {
        self.leave_scope();
    }
    fn visit_grid_row(&mut self, g: &GridRowDefinitions) {
        self.leave_scope();
    }
    fn visit_col_def(&mut self, g: &ColumnDefinition) {
        self.leave_scope();
    }
    fn visit_row_def(&mut self, g: &RowDefinition) {
        self.leave_scope();
    }
    fn visit_content_page(&mut self, g: &ContentPage) {
        self.leave_scope();
    }
    fn visit_unknown(&mut self, g: &Unknown) {
        self.leave_scope();
    }
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
