use parking_lot::Mutex;

use crate::ui_elements::UITreeRef;


pub trait CallableByName {
    fn call_method(&self, name: &str);
    fn get_tree(&self) -> Option<UITreeRef>;
    fn set_tree(&mut self, tree: Option<UITreeRef>);
}

pub type MainCallable = &'static Mutex<dyn CallableByName>;
