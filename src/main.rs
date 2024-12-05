use std::{env, io};
use std::io::Write;
mod gates;
mod adder; 
mod memory;
mod utils;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mem = memory::RAM8::new();

    loop {
        loop {

            print!("nandputer> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();

            match input {
                "m" => println!("{:?}", mem),
                _   => println!("Invalid input"),
            }
        }
    }
}
