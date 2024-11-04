use std::io;

const INPUT: &str = "";
const NUM1: &str = "";
const NUM2: &str = "";
const OPERATION: &str = "";

// NOTE: maximum I8/16/32/64 variables
const MAX_I8: i8 = i8::MAX;
const MAX_I16: i16 = i16::MAX;
const MAX_I32: i32 = i32::MAX;
const MAX_I64: i64 = i64::MAX;

// NOTE: maximum U8/16/32/64 variables
const MAX_U8: u8 = u8::MAX;
const MAX_U16: u16 = u16::MAX;
const MAX_U32: u32 = u32::MAX;
const MAX_U64: u64 = u64::MAX;

// NOTE: minimum I8/16/32/64/128 variables
const MIN_I8: i8 = i8::MIN;
const MIN_I16: i16 = i16::MIN;
const MIN_I32: i32 = i32::MIN;
const MIN_I64: i64 = i64::MIN;
const MIN_I128: i128 = i128::MIN;

fn perform_operation() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operation = String::new();

    println!("1. Add (+)");
    println!("2. Subtract (-)");
    println!("3. Multiply (*)");
    println!("4. Divide (/)");
    println!("Select operation:");
    if let Ok(_) = io::stdin().read_line(&mut operation) {
        println!("You selected {}", operation.trim());
    } else {
        println!("There was an error while getting input");
        return;
    }

    println!("Select first value (format: i8, u16, min_i16, etc):");
    if let Ok(_) = io::stdin().read_line(&mut num1) {
        num1 = num1.trim().to_string();
    } else {
        println!("There was an error while getting input");
        return;
    }

    println!("Select second value (format: i8, u16, min_i16, etc):");
    if let Ok(_) = io::stdin().read_line(&mut num2) {
        num2 = num2.trim().to_string();
    } else {
        println!("There was an error while getting input");
        return;
    }

    let value1: i128 = match num1.as_str() {
        "i8" => MAX_I8 as i128,
        "i16" => MAX_I16 as i128,
        "i32" => MAX_I32 as i128,
        "i64" => MAX_I64 as i128,
        "u8" => MAX_U8 as i128,
        "u16" => MAX_U16 as i128,
        "u32" => MAX_U32 as i128,
        "u64" => MAX_U64 as i128,
        "min_i8" => MIN_I8 as i128,
        "min_i16" => MIN_I16 as i128,
        "min_i32" => MIN_I32 as i128,
        "min_i64" => MIN_I64 as i128,
        _ => {
            println!("Invalid input for num1");
            return;
        }
    };

    let value2: i128 = match num2.as_str() {
        "i8" => MAX_I8 as i128,
        "i16" => MAX_I16 as i128,
        "i32" => MAX_I32 as i128,
        "i64" => MAX_I64 as i128,
        "u8" => MAX_U8 as i128,
        "u16" => MAX_U16 as i128,
        "u32" => MAX_U32 as i128,
        "u64" => MAX_U64 as i128,
        "min_i8" => MIN_I8 as i128,
        "min_i16" => MIN_I16 as i128,
        "min_i32" => MIN_I32 as i128,
        "min_i64" => MIN_I64 as i128,
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
}


fn maximum_values() {
    println!("Maximum values are:");
    println!("For signed:
            i8: {}\ni16: {}\ni32: {}\ni64: {}",
             MAX_I8, MAX_I16, MAX_I32, MAX_I64);
    println!("For unsigned:
            u8: {}\nu16: {}\nu32: {}\nu64: {}",
             MAX_U8, MAX_U16, MAX_U32, MAX_U64);
}

fn minimum_values() {
    println!("Minimum values are:");
    println!("For signed:
            i8: {}\ni16: {}\ni32: {}\ni64: {}\ni128: {}",
             MIN_I8, MIN_I16, MIN_I32, MIN_I64, MIN_I128);
}

fn main() {
    let mut input = String::new();
    println!("--- ACTIONS ---\n1 - get maximum values\n2 - get minimum values\n3 - perform operation with max values \nSelect action:");
    if let Ok(_) = io::stdin().read_line(&mut input) {
        match input.trim() {
            "1" => maximum_values(),
            "2" => minimum_values(),
            "3" => perform_operation(),
            _ => println!("Invalid action selected"),
        }
    } else {
        println!("There was an error while getting input");
    }
}