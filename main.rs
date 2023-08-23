mod alu;
mod pc;
mod utils;

fn main() {
    // TODO: make initializer

    // Makes PC
    /*let pc = pc::PC {
        curr_instruction_index: String::from("000000000000")
    };*/

    // TODO: make instruction memory

    // TODO: make data memory (Should use same memory as instructions memory)

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