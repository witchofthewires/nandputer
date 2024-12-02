use std::env;
mod gates;
mod adder; 
mod memory;

fn main() {

    let args: Vec<String> = env::args().collect();
    let val1 = &args[1];
    let val2 = &args[2];
    let func = &args[3];

    let val1 = match &val1[..] {
        "0" => false,
        "1" => true,
        _   => {
            println!("Invalid arg1");
            return();
        }
    };

    let val2 = match &val2[..] {
        "0" => false,
        "1" => true,
        _   => {
            println!("Invalid arg2");
            return();
        }
    };

    let result = match &func[..] {
        "nand" => gates::nand(val1, val2),
        "not"  => gates::not(val1),
        "and"  => gates::and(val1, val2),
        "or"   => gates::or(val1, val2),
        "nor"  => gates::nor(val1, val2),
        "xor"  => gates::xor(val1, val2),
        _      => {
            println!("Invalid func");
            return();
        }
    };

    println!("{} {} {} = {}", val1, func, val2, result);
}
