
pub struct Event {
    name: String
}

impl Event {
    pub fn new(name: &str) -> Event {
        Event {
            name: name.to_string()
        }
    }
}