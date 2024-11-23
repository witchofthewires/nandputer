use std::env;

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
        "nand" => nand(val1, val2),
        "not"  => not(val1),
        "and"  => and(val1, val2),
        "or"   => or(val1, val2),
        "nor"  => nor(val1, val2),
        "xor"  => xor(val1, val2),
        _      => {
            println!("Invalid func");
            return();
        }
    };

    println!("{} {} {} = {}", val1, func, val2, result);
}

fn nand(val1: bool, val2: bool) -> bool {
    !(val1 & val2)
}

fn not(val: bool) -> bool {
    nand(val, val)
}

fn and(val1: bool, val2: bool) -> bool {
    not(nand(val1, val2))
}

fn or(val1: bool, val2: bool) -> bool {
    nand(not(val1), not(val2))
}

fn nor(val1: bool, val2: bool) -> bool {
    not(or(val1, val2))
}

fn xor(val1: bool, val2: bool) -> bool {
    and(or(val1, val2), nand(val1, val2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_works() {
        assert_eq!(nand(false, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(true, true), false);
    }
    
    #[test]
    fn test_not_works() {
        assert_eq!(not(false), true);
        assert_eq!(not(true), false);
    }

    #[test]
    fn test_and_works() {
        assert_eq!(and(false, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(true, false), false);
        assert_eq!(and(true, true), true);
    }

    #[test]
    fn test_or_works() {
        assert_eq!(or(false, false), false);
        assert_eq!(or(false, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(true, true), true);
    }

    #[test]
    fn test_nor_works() {
        assert_eq!(nor(false, false), true);
        assert_eq!(nor(false, true), false);
        assert_eq!(nor(true, false), false);
        assert_eq!(nor(true, true), false);
    }

    #[test]
    fn test_xor_works() {
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(true, true), false);
    }
}