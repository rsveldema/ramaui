
use std::rc::Rc;
use std::sync::Mutex;

use gtk::gio::ApplicationFlags;
use gtk::{glib, prelude::*};
use gtk::{Application, ApplicationWindow};

use crate::stack_layout::StackLayout;
use crate::ui_elements::UIElementRef;
use crate::visitor::Visitor;
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

fn open_ui<T>(app: &Application, main_win: &Rc<Mutex<T>>, files: &[gtk::gio::File], _s: &str, root: UIElementRef) {
    for file in files {
        println!("Open--: {:?}", file.path());
    }

    let window_opt = build_ui_from_xaml::<T>(app, root, main_win);
    if let Some(window) = window_opt {
        window.present();
    } else {
        panic!("failed to build ui from XAML");
    }
}

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

struct UIBuilder<'b, T> {
    root: Option<ApplicationWindow>,
    app: &'b Application,
    nested_gtk_items: Vec<Vec<GtkPushed>>,
    main_win: &'b Rc<Mutex<T>>
}

impl<'b, T> UIBuilder<'b, T> {
    fn new(app: &'b Application, win: &'b Rc<Mutex<T>>) -> UIBuilder<'b, T> {
        return UIBuilder::<T> {
            root: Option::None,
            app: app,
            nested_gtk_items: Vec::new(),
            main_win: win
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

impl<'lifetime, T> Visitor for UIBuilder<'lifetime, T> {
    fn start_visit_button(&mut self, _b: &Button) {
        self.enter_scope()
    }
    fn start_visit_window(&mut self, _w: &Window) {
        self.enter_scope()
    }
    fn start_visit_label(&mut self, _l: &Label) {
        self.enter_scope()
    }
    fn start_visit_text_block(&mut self, _t: &TextBlock) {
        self.enter_scope()
    }
    fn start_visit_grid(&mut self, _g: &GridLayout) {
        self.enter_scope()
    }
    fn start_visit_grid_cols(&mut self, _g: &GridColumnDefinitions) {
        self.enter_scope()
    }
    fn start_visit_grid_row(&mut self, _g: &GridRowDefinitions) {
        self.enter_scope()
    }
    fn start_visit_col_def(&mut self, _g: &ColumnDefinition) {
        self.enter_scope()
    }
    fn start_visit_row_def(&mut self, _g: &RowDefinition) {
        self.enter_scope()
    }
    fn start_visit_content_page(&mut self, _g: &ContentPage) {
        self.enter_scope()
    }
    fn start_visit_unknown(&mut self, _g: &Unknown) {
        self.enter_scope()
    }
    fn start_visit_stack(&mut self, _g: &StackLayout) {
        self.enter_scope()
    }

    fn visit_button(&mut self, b: &Button) {
        let gtk_b = gtk::Button::with_label(&b.get_text());
        gtk_b.connect_clicked(|_b| {
          
        });
        self.leave_scope();
        self.last_scope().push(GtkPushed::Button(gtk_b));
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
        let gtk_label = gtk::Label::new(Option::Some(l.get_content().as_str()));

        self.leave_scope();
        self.last_scope().push(GtkPushed::Label(gtk_label));
    }
    
    fn visit_text_block(&mut self, _t: &TextBlock) {
        self.leave_scope();
    }
    
    fn visit_grid(&mut self, _g: &GridLayout) {
        self.leave_scope();
    }
    
    fn visit_stack(&mut self, _g: &StackLayout) {
        self.leave_scope();
    }
    
    fn visit_grid_cols(&mut self, _g: &GridColumnDefinitions) {
        self.leave_scope();
    }
    
    fn visit_grid_row(&mut self, _g: &GridRowDefinitions) {
        self.leave_scope();
    }
    
    fn visit_col_def(&mut self, _g: &ColumnDefinition) {
        self.leave_scope();
    }
    
    fn visit_row_def(&mut self, _g: &RowDefinition) {
        self.leave_scope();
    }
    
    fn visit_content_page(&mut self, _g: &ContentPage) {
        self.leave_scope();
    }
    
    fn visit_unknown(&mut self, _g: &Unknown) {
        self.leave_scope();
    }
}


pub fn build_ui_from_xaml<T>(app: &Application, root: UIElementRef, main_win: &Rc<Mutex<T>>) -> Option<ApplicationWindow> {
    let mut builder = UIBuilder::<T>::new(app, main_win);
    root.as_ref().lock().unwrap().visit(&mut builder);
    return builder.root;
}


pub fn start_interpreter<T: 'static>(root: &UIElementRef, win: &Rc<Mutex<T>>) -> glib::ExitCode {
    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    // clone to get rid of the borrow of the parameter
    let k = root.clone();
    let z = win.clone();
    app.connect_open(
        move |app: &Application, files: &[gtk::gio::File], s: &str| {
            // clone k to pass into open-ui
            open_ui::<T>(app, &z, files, s, k.clone());
        },
    );

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}