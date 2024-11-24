use std::env;
use std::iter;

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

fn not16(val: &Vec<bool>) -> Vec<bool> { 
    let mut res = Vec::new();
    for bit in val {
        res.push(not(*bit));
    }
    res
}

fn and(val1: bool, val2: bool) -> bool {
    not(nand(val1, val2))
}

fn and16(val1: &Vec<bool>, val2: &Vec<bool>) -> Vec<bool> { 
    let mut res = Vec::new();
    for (bit1, bit2) in iter::zip(val1, val2) {
        res.push(and(*bit1, *bit2));
    }
    res
}

fn or(val1: bool, val2: bool) -> bool {
    nand(not(val1), not(val2))
}

fn or16(val1: &Vec<bool>, val2: &Vec<bool>) -> Vec<bool> { 
    let mut res = Vec::new();
    for (bit1, bit2) in iter::zip(val1, val2) {
        res.push(or(*bit1, *bit2));
    }
    res
}

fn nor(val1: bool, val2: bool) -> bool {
    not(or(val1, val2))
}

fn xor(val1: bool, val2: bool) -> bool {
    and(or(val1, val2), nand(val1, val2))
}

fn mux(val1: bool, val2: bool, sel: bool) -> bool {
    or(and(val1, not(sel)), and(val2, sel))    
}

fn mux16(val1: &Vec<bool>, val2: &Vec<bool>, sel: bool) -> Vec<bool> { 
    let mut res = Vec::new();
    for (bit1, bit2) in iter::zip(val1, val2) {
        res.push(mux(*bit1, *bit2, sel));
    }
    res
}

fn dmux(val: bool, sel: bool) -> (bool, bool) {
    (and(val, not(sel)), 
     and(val, sel))
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

    #[test]
    fn test_mux_works() {
        assert_eq!(mux(false, false, false), false);
        assert_eq!(mux(false, true, false), false);
        assert_eq!(mux(true, false, false), true);
        assert_eq!(mux(true, true, false), true);
        assert_eq!(mux(false, false, true), false);
        assert_eq!(mux(false, true, true), true);
        assert_eq!(mux(true, false, true), false);
        assert_eq!(mux(true, true, true), true);
    }

    #[test]
    fn test_dmux_works() {
        assert_eq!(dmux(false, false), (false, false));
        assert_eq!(dmux(false, true), (false, false));
        assert_eq!(dmux(true, false), (true, false));
        assert_eq!(dmux(true, true), (false, true));
    }

    #[test]
    fn test_not16_works() {
        let val = vec![false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let res = vec![true, false, true, false, false, false, true, false, true, true, true, false, false, true, false, false];
        assert_eq!(not16(&val), res);
    }

    #[test]
    fn test_and16_works() {
        let val1 = vec![false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let val2 = vec![false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, false];
        let res =  vec![false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, false];
        assert_eq!(and16(&val1, &val2), res);
    }

    #[test]
    fn test_or16_works() {
        let val1 = vec![false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let val2 = vec![false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, false];
        let res =  vec![false, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true];
        assert_eq!(or16(&val1, &val2), res);
    }

    #[test]
    fn test_mux16_works() {
        let val1 = vec![false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let val2 = vec![false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, false];
        assert_eq!(mux16(&val1, &val2, false), val1);
        assert_eq!(mux16(&val1, &val2, true), val2);
    }
}