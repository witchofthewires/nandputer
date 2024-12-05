use std::{env, io};
use std::io::Write;
mod gates;
mod adder; 
mod memory;
mod utils;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut mem = memory::RAM8::new();
    let mut val: u8 = 0;

    loop {

        print!("nandputer> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        //let addr: u8 = 3;
        match input {
            "m" => println!("{}", mem),
            "w" => {
                        //val += 1;
                        for i in 0..8 {
                            let in_bits = utils::bytes_to_boollist(&[0,i]);
                            let addr_bits = utils::bytes_to_boollist(&[0,i]);
                            mem.cycle(&in_bits, &addr_bits, true);
                            println!("Wrote {} to {}", i, i)
                        }
            }
            _   => println!("Invalid input"),
        }
    }
}
