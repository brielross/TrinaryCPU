mod alu;
mod pc;
mod utils;
mod memory;

use std::collections::HashMap;

fn main() {
    // TODO: make initializer

    // Makes PC
    /*let pc = pc::PC {
        curr_instruction_index: String::from("000000000000")
    };*/

    // Makes memory (both instruction and data)
    /*let mut memory = memory::Memory {
        memory_data: HashMap::new()
    };

    memory.update_memory(String::from("00"), String::from("000"));
    memory.update_memory(String::from("01"), String::from("001"));
    memory.update_memory(String::from("02"), String::from("002"));

    println!(
        "{}, {}, {}", 
        memory.access_memory(&String::from("00")), 
        memory.access_memory(&String::from("01")), 
        memory.access_memory(&String::from("02"))
    )*/

    // TODO: make registers

    // ALU test
    /*let alu = alu::ALU {
        input_1: String::from("00000002111"),
        input_2: String::from("000000000222"),
        operation: String::from("0")
    };
    let result = alu.calc();
    println!(
        "{}, {}, {}",
        utils::trip_string_to_decimal(String::from("00000002111")),
        utils::trip_string_to_decimal(String::from("000000000222")),
        utils::trip_string_to_decimal(result)
    )*/

    // TODO: make controller

    // TODO: make datapath loop
}