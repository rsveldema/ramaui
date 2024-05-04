use std::rc::Rc;
use std::sync::Mutex;

use gtk::gio::ApplicationFlags;
use gtk::{glib, prelude::*};
use gtk::{Application, ApplicationWindow};

use crate::callable::CallableByName;
use crate::stack_layout::StackLayout;
use crate::ui_elements::{UIElement, UIElementRef};
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

fn open_ui<'b>(
    app: &'b Application,
    main_win: Rc<Mutex<&'static dyn CallableByName>>,
    files: &[gtk::gio::File],
    _s: &str,
    root: UIElementRef,
) {
    for file in files {
        println!("Open--: {:?}", file.path());
    }

    let builder = build_ui_from_xaml(app, root, main_win);
    let window_opt = builder.root;
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

struct UIBuilder<'b> {
    root: Option<ApplicationWindow>,
    app: &'b Application,
    nested_gtk_items: Vec<Vec<GtkPushed>>,
    _main_win: Rc<Mutex<&'static dyn CallableByName>>,
}

impl<'b> UIBuilder<'b> {
    fn new(app: &'b Application, win: Rc<Mutex<&'static dyn CallableByName>>) -> UIBuilder<'b> {
        UIBuilder {
            root: Option::None,
            app,
            nested_gtk_items: Vec::new(),
            _main_win: win.clone(),
        }
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
        let clicked_opt = b.get_attribute(&"Click".to_string());
        let gtk_b = gtk::Button::with_label(&b.get_text());
        if let Some(clicked) = clicked_opt {
            let mw = self._main_win.clone();
            gtk_b.connect_clicked(move |_b| {
                let w = mw.lock();
                match w {
                    Ok(z) => {
                        z.call_method(&clicked);
                    }

                    Err(e) => {
                        panic!("failed to lock main win: {}", e);
                    }
                }
            });
        }

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

fn build_ui_from_xaml<'b>(
    app: &'b Application,
    root: UIElementRef,
    main_win: Rc<Mutex<&'static dyn CallableByName>>,
) -> UIBuilder<'b> {
    let mut builder = UIBuilder::new(app, main_win.clone());
    root.as_ref().lock().unwrap().visit(&mut builder);
    builder
}

pub fn start_interpreter(
    root: &UIElementRef,
    win: Rc<Mutex<&'static dyn CallableByName>>,
) -> glib::ExitCode {
    // Create a new application
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    // clone to get rid of the borrow of the parameter
    let root_clone: Rc<Mutex<dyn crate::ui_elements::UIElement>> = root.clone();
    let win_clone = win.clone();
    app.connect_open(
        move |app: &Application, files: &[gtk::gio::File], s: &str| {
            // clone k to pass into open-ui
            open_ui(app, win_clone.clone(), files, s, root_clone.clone());
        },
    );

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();

    panic!();
}
