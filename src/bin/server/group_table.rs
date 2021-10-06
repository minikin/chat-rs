use crate::chat_group::ChatGroup;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct GroupTable(Mutex<HashMap<Arc<String>, Arc<ChatGroup>>>);

impl GroupTable {
    pub fn new() -> GroupTable {
        GroupTable(Mutex::new(HashMap::new()))
    }

    pub fn get(&self, name: &str) -> Option<Arc<ChatGroup>> {
        self.0.lock().unwrap().get(&name.to_string()).cloned()
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
