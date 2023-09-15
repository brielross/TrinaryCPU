use std::collections::HashMap;

pub struct Memory {
    pub memory_data: HashMap<String, String>
}

impl Memory {
    pub fn update_memory(&mut self, index: String, value: String) {
        self.memory_data.insert(index, value);
    }

    pub fn access_memory(&mut self, index: &String) -> &String {
        return &self.memory_data[index];
    }

    pub fn read_registers(&mut self, first_index: &String, second_index : &String) -> (&String, &String) {
        return (&self.memory_data[first_index], &self.memory_data[second_index]);
    }
}