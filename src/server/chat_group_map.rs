use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::ChatGroup;

pub struct ChatGroupMap(Mutex<HashMap<Arc<String>, Arc<ChatGroup>>>);

impl ChatGroupMap {
    pub fn new() -> Self {
        ChatGroupMap(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, name: &String) -> Option<Arc<ChatGroup>> {
        self.0.lock().unwrap().get(name).cloned()
    }

    pub fn get_or_create(&self, name: Arc<String>) -> Arc<ChatGroup> {
        self.0
            .lock()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(|| Arc::new(ChatGroup::new(name)))
            .clone()
    }
}
