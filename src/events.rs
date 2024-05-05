use crate::callable::MainCallable;


pub struct Event {
    name: String, 
    mc: MainCallable
}

impl Event {
    pub fn new(name: &str, mc: MainCallable) -> Event {
        Event {
            name: name.to_string(),
            mc
        }
    }

    pub fn get_callable(&self) -> MainCallable {
        return self.mc;
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}