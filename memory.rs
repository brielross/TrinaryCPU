use std::collections::HashMap;

pub struct Memory {
    pub memory_data: HashMap<String, String>
}

impl Memory {
    pub fn update_memory(&mut self, index: String, value: String) {
        self.memory_data.entry(index).or_insert(value);
    }

    pub fn access_memory(&self, index: &String) -> &String {
        return &self.memory_data[index];
    }
}