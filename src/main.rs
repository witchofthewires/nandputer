use std::{env, io, process, iter};
use std::io::Write;
mod gates;
mod adder; 
mod memory;
mod utils;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut mem = memory::RAM64::new();
    let mut val: u8 = 0;

    loop {

        print!("nandputer> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input_switch = input.trim();

        //let addr: u8 = 3;
        match input_switch {
            "m" => {
                        for (i, word) in iter::zip(0..64, mem) { 
                            if i % 4 == 0 { print!("\n{i}: "); }
                            print!("\t{word}");
                        }
                        println!();
            }
            "w" => {
                        print!("addr# ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read addr");
                        let addr: u16 = input.trim().parse().expect("Invalid addr input");

                        print!("data# ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read data");
                        let data: u16 = input.trim().parse().expect("Invalid data input");

                        let in_bits = utils::bytes_to_boollist(&utils::split_u16(data));
                        let addr_bits = utils::bytes_to_boollist(&utils::split_u16(addr));
                        mem.clk_cycle(&in_bits, &addr_bits, true);
                        println!("Wrote {} to {}", data, addr);
                    }
            "q" => { println!("Terminating..."); process::exit(0); }
            _   => println!("Invalid input"),
        }
    }
}
