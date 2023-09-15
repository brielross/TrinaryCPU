mod alu;
mod pc;
mod utils;
mod memory;
mod controller;

use std::collections::HashMap;

fn main() {
    let debug = true;

    // Initialization
    let mut pc = pc::PC {
        curr_instruction_index: "000000000000".to_string()
    };

    // 12 trip address and output
    // TODO: bring in instructions
    /*let mut instruction_memory = memory::Memory {
        memory_data: HashMap::new()
    };*/

    // 12 trip address and output
    let mut data_memory = memory::Memory {
        memory_data: HashMap::new()
    };

    // 3 trip addresses and 12 trip output
    let mut registers = memory::Memory {
        memory_data: HashMap::new()
    };
    // Set zero and one registers as constants
    registers.update_memory("000".to_string(), "000000000000".to_string());
    registers.update_memory("001".to_string(), "000000000001".to_string());

    // Testing data
    registers.update_memory("002".to_string(), "000111100001".to_string());
    registers.update_memory("021".to_string(), "000100000001".to_string());
    registers.update_memory("100".to_string(), "000010000001".to_string());
    registers.update_memory("111".to_string(), "001001000001".to_string());
    data_memory.update_memory("0012101".to_string(), "002002000002".to_string());

    let mut end_of_instructions = false;
    while !end_of_instructions {
        // PC+1
        let curr_instruction_index = pc.get_curr_instruction_index();

        // Get instruction
        // TODO: fetch from memory and set end of instruction to true if no instruction in memory
        let curr_instruction = "220000001000".to_string();
        if debug { 
            println!("Debug for instruction {} at index {}:", curr_instruction, curr_instruction_index);
        }

        let pc_alu = alu::ALU {
            input_1: curr_instruction_index.to_string(),
            input_2: "000000000001".to_string(),
            operation: '0'
        };
        let pc_plus_one = pc_alu.calc();
        if debug {
            println!("\tCalculating pc+1 = {}", pc_plus_one);
        }

        // Fetch from registers
        if debug {
            println!("\tReading from registers {} and {}",&utils::substring(&curr_instruction, 2, 5), &utils::substring(&curr_instruction, 5, 8));
        }
        let (read_data_one, read_data_two) = registers.read_registers(
            &utils::substring(&curr_instruction, 2, 5),
            &utils::substring(&curr_instruction, 5, 8)
        );

        // Control unit
        let codes = controller::get_control_codes(
            utils::substring(&curr_instruction, 0, 2),
            utils::trip_string_is_zero(read_data_two.to_string())
        );

        // ALU
        let alu = alu::ALU {
            input_1: read_data_one.to_string(),
            input_2: read_data_two.to_string(),
            operation: codes.alu_code
        };
        let alu_result = alu.calc();
        if debug {
            println!("\tResult from alu with operation = {}, input 1 = {}, and input 2 = {}: {}", codes.alu_code, read_data_one, read_data_two, alu_result);
        }

        // Mem access
        let mut mem_read_value = "";
        if codes.mem_read_write == '1' {
            // Read
            mem_read_value = data_memory.access_memory(&utils::substring(&curr_instruction, 5, 12));
            if debug {
                println!("\tMemory read of {} gave {}", utils::substring(&curr_instruction, 5, 12), mem_read_value);
            }
        } else if codes.mem_read_write == '2' {
            // Write
            if debug {
                println!("\tWriting {} to memory index {}", read_data_one, utils::substring(&curr_instruction, 5, 12));
            }
            data_memory.update_memory(utils::substring(&curr_instruction, 5, 12), read_data_one.to_string());
        }

        // Update PC
        if codes.pc_src == '0' {
            if debug {
                println!("\tUsing pc+1 = {} as next address", pc_plus_one);
            }
            pc.set_instruction_index(pc_plus_one);
        } else if codes.pc_src == '1' {
            if debug {
                println!("\tUsing immediate = {} as next address", "00".to_string() + utils::substring(&curr_instruction, 2, 12).as_str());
            }
            pc.set_instruction_index("00".to_string() + utils::substring(&curr_instruction, 2, 12).as_str());
        } else if codes.pc_src == '2' {
            if debug {
                println!("\tUsing read date one = {} as next address", read_data_one.to_string());
            }
            pc.set_instruction_index(read_data_one.to_string());
        }

        // Reg write
        if codes.reg_write == '2' {
            let reg_write_data: String;
            if codes.reg_write_data == '0' {
                if debug {
                    println!("\tReg write data = alu result = {}", alu_result);
                }
                reg_write_data = alu_result;
            } else if codes.reg_write_data == '1' {
                if debug {
                    println!("\tReg write data = immediate = {}", utils::substring(&curr_instruction, 2, 12));
                }
                reg_write_data = utils::substring(&curr_instruction, 2, 12);
            } else {
                if debug {
                    println!("\tReg write data = mem read data = {}", mem_read_value);
                }
                reg_write_data = mem_read_value.to_string();
            }

            let reg_write_src: String;
            if codes.write_reg_scr == '0' {
                if debug {
                    println!("\tReg write src = from instruction = {}", utils::substring(&curr_instruction, 8, 11));
                }
                reg_write_src = utils::substring(&curr_instruction, 8, 11);
            } else {
                if debug {
                    println!("\tReg write src = trip 111");
                }
                reg_write_src = "111".to_string();
            }

            registers.update_memory(reg_write_src, reg_write_data)
        }

        // TODO: figure out how to get end of instruction
        end_of_instructions = true;
    }

    println!("\n\nResults:");
    println!("PC: {}", pc.curr_instruction_index);
    println!("Registers:");
    for (key, value) in registers.memory_data {
        let decimal_value = utils::trip_string_to_decimal(value.clone());
        let register_decimal = utils::trip_string_to_decimal(key.clone());
        println!("Register {} = {}: (Trips: {}, Decimal: {})", key, register_decimal, value, decimal_value);
    }
    println!("Data Memory:");
    for (key, value) in data_memory.memory_data {
        let decimal_value = utils::trip_string_to_decimal(value.clone());
        println!("Index {}: (Trips: {}, Decimal: {})", key, value, decimal_value);
    }
}
