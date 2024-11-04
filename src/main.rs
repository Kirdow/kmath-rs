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

struct InputData {
    input_type: InputType,
    precision: Option<u32>
}

impl InputData {
    pub fn new() -> Self {
        let mut input_type: Option<String> = None;
        let mut precision: Option<u32> = None;

        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            if arg == "--precision" || arg == "-p" {
                if let Some(next_arg) = args.next() {
                    if let Ok(value) = next_arg.parse::<u32>() {
                        precision = Some(value);
                    } else {
                        eprintln!("Error: Expected a precision number after {}, but got {}", arg, next_arg);
                    }
                } else {
                    eprintln!("Error: Expected a value after {}", arg);
                }
            } else if !arg.starts_with("--") {
                input_type = Some(arg);
            }
        }

        let input_type = if let Some(arg) = input_type {
            match arg.parse::<f32>() {
                Ok(num) => InputType::Args(num),
                Err(_) => InputType::Input(ask_for_num())
            }
        } else {
            InputType::Input(ask_for_num())
        };

        Self {
            input_type: input_type,
            precision: precision
        }
    }

    pub fn get_value(&self) -> f32 {
        self.input_type.get_value()
    }
}

impl InputType {
    pub fn get_value(&self) -> f32 {
        match self {
            InputType::Args(f) => *f,
            InputType::Input(f) => *f
        }
    }
}

fn main() {
    let input_data = InputData::new();
    let input = input_data.get_value().mf();

    if let Some(prec) = input_data.precision {
        kmath::set_precision(prec);
    }

    let start = Instant::now();
    let result = kmath::ksqrt(input.clone());
    let elapsed_ns = start.elapsed().as_nanos();

    if let InputType::Input(_) = input_data.input_type {
        println!("Calculating square root of: {}", input.mp());
        println!("Process took: {} ms", (elapsed_ns as f64) * 1e-6_f64);
        println!("Result:\n{}", result.clone().mp());
    } else {
        println!("{}", result.clone().mp());
    }
}
