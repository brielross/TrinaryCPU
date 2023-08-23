mod alu;
mod utils;

fn main() {
    let alu = alu::ALU {
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
    )
}