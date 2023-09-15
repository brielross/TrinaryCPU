pub struct PC {
    pub curr_instruction_index: String
}

impl PC {
    pub fn set_instruction_index(&mut self, next_instruction: String) {
        self.curr_instruction_index = next_instruction;
    }

    pub fn get_curr_instruction_index(&self) -> &String {
        return &self.curr_instruction_index;
    }
}