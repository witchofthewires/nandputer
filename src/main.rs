use std::{env, io, process, iter};
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
            "m" => {
                        for (i, word) in iter::zip(0..8, mem) { println!("{i}: {word}"); }
            }
            "w" => {
                        //val += 1;
                        for i in 0..8 {
                            let in_bits = utils::bytes_to_boollist(&[0,i]);
                            let addr_bits = utils::bytes_to_boollist(&[0,i]);
                            mem.clk_cycle(&in_bits, &addr_bits, true);
                            println!("Wrote {} to {}", i, i)
                        }
                    }
            "q" => { println!("Terminating..."); process::exit(0); }
            _   => println!("Invalid input"),
        }
    }
}
