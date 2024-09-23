use std::io;

fn main() {
    let mut input = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operation = String::new();

    let max_i8: i8 = i8::MAX;
    let max_i16: i16 = i16::MAX;
    let max_i32: i32 = i32::MAX;
    let max_i64: i64 = i64::MAX;
    let max_i128: i128 = i128::MAX;

    let max_u8: u8 = u8::MAX;
    let max_u16: u16 = u16::MAX;
    let max_u32: u32 = u32::MAX;
    let max_u64: u64 = u64::MAX;
    let max_u128: u128 = u128::MAX;

    println!("--- ACTIONS ---");
    println!("1 - get maximum values");
    println!("2 - perform operation with max values");
    println!("Select action:");
    io::stdin().read_line(&mut input).expect("There was an error while getting input");

    if input.trim() == "1" {
        println!("Signed:");
        println!(
            "i8: {}\ni16: {}\ni32: {}\ni64: {}\ni128: {}",
            max_i8, max_i16, max_i32, max_i64, max_i128
        );

        println!("");

        println!("Unsigned:");
        println!(
            "u8: {}\nu16: {}\nu32: {}\nu64: {}\nu128: {}",
            max_u8, max_u16, max_u32, max_u64, max_u128
        );
    } else if input.trim() == "2" {
        println!("1. Add (+)");
        println!("2. Subtract (-)");
        println!("3. Multiply (*)");
        println!("4. Divide (/)");
        println!("Select operation:");
        io::stdin().read_line(&mut operation).expect("There was an error while getting input");
        println!("You selected {}", operation);

        println!("1. i(8/16/32/64)");
        println!("2. u(8/16/32/64)");
        println!("Select first value (format: i8, u16, etc):");
        io::stdin().read_line(&mut num1).expect("There was an error while getting input");
        let num1 = num1.trim();

        println!("Select second value (format: i8, u16, etc):");
        io::stdin().read_line(&mut num2).expect("There was an error while getting input");
        let num2 = num2.trim();

        let value1: i128 = match num1 {
            "i8" => max_i8 as i128,
            "i16" => max_i16 as i128,
            "i32" => max_i32 as i128,
            "i64" => max_i64 as i128,
            "u8" => max_u8 as i128,
            "u16" => max_u16 as i128,
            "u32" => max_u32 as i128,
            "u64" => max_u64 as i128,
            _ => {
                println!("Invalid input for num1");
                return;
            }
        };

        let value2: i128 = match num2 {
            "i8" => max_i8 as i128,
            "i16" => max_i16 as i128,
            "i32" => max_i32 as i128,
            "i64" => max_i64 as i128,
            "u8" => max_u8 as i128,
            "u16" => max_u16 as i128,
            "u32" => max_u32 as i128,
            "u64" => max_u64 as i128,
            _ => {
                println!("Invalid input for num2");
                return;
            }
        };

        let result = match operation.trim() {
            "1" => value1 + value2,
            "2" => value1 - value2,
            "3" => value1 * value2,
            "4" => {
                if value2 != 0 {
                    value1 / value2
                } else {
                    println!("Cannot divide by zero");
                    return;
                }
            }
            _ => {
                println!("Invalid operation selected");
                return;
            }
        };

        println!("Result of the operation: {}", result);
    } else {
        println!("Invalid action selected");
    }
}
