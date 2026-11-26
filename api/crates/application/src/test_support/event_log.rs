use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct EventLog(Arc<Mutex<Vec<&'static str>>>);

impl EventLog {
    pub fn record(&self, event: &'static str) {
        self.0.lock().expect("events lock").push(event);
    }

    pub fn events(&self) -> Vec<&'static str> {
        self.0.lock().expect("events lock").clone()
    }
}
