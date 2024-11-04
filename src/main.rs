mod kmath;

use std::time::Instant;
use kmath::{MakeFloat, MakePrintable};
use std::io::{self, Write};
use std::env;

fn ask_for_num() -> f32 {
    loop {
        print!("Please enter number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        match input.parse::<f32>() {
            Ok(number) => {
                return number;
            },
            Err(_) => {
                println!("Invalid number. Try again!");
            }
        }
    }
}

enum InputType {
    Args(f32),
    Input(f32)
}

fn get_input() -> InputType {
    let args: Vec<String> = env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.parse::<f32>() {
            Ok(num) => InputType::Args(num),
            Err(_) => InputType::Input(ask_for_num())
        }
    } else {
        InputType::Input(ask_for_num())
    }
}

fn get_input_value(it: &InputType) -> f32 {
    match it {
        InputType::Args(f) => *f,
        InputType::Input(f) => *f
    }
}

fn main() {
    let input_type = get_input();
    let input = get_input_value(&input_type).mf();
    let start = Instant::now();
    let result = kmath::ksqrt(input.clone());
    let elapsed_ns = start.elapsed().as_nanos();

    if let InputType::Input(_) = input_type {
        println!("Calculating square root of: {}", input.mp());
        println!("Process took: {} ms", (elapsed_ns as f64) * 1e-6_f64);
        println!("Result:\n{}", result.clone().mp());
    } else {
        println!("{}", result.clone().mp());
    }
}
