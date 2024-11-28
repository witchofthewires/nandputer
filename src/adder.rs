use std::iter;

use crate::gates;

fn half_adder(bit1: bool, bit2: bool) -> (bool, bool) {
    (gates::xor(bit1, bit2), 
     gates::and(bit1, bit2))
}

fn full_adder(bit1: bool, bit2: bool, carry: bool) -> (bool, bool) {
    let (half_sum, half_carry_1) = half_adder(bit1, bit2);
    let (full_sum, half_carry_2) = half_adder(half_sum, carry);
    (full_sum, gates::xor(half_carry_1, half_carry_2))
}

// carry-ripple adder
// TODO replace with carry-lookahead
fn add16(val1: &Vec<bool>, val2: &Vec<bool>) -> Vec<bool> {
    let mut res: [bool; 16] = [false; 16];
    let mut carry_bus: [bool; 17] = [false; 17]; // prevent bus overflow on final bit
    let mut rev_val1 = val1.clone();
    let mut rev_val2 = val2.clone();
    rev_val1.reverse();
    rev_val2.reverse();
    let mut i = 0;
    for (bit1, bit2) in iter::zip(rev_val1, rev_val2) {
        (res[i], carry_bus[i+1]) = full_adder(bit1, bit2, carry_bus[i]);
        i += 1;
    }
    res.reverse();
    Vec::from(res)
}

#[cfg(test)]
mod tests {
    use gates::bytes_to_boolvec;

    use super::*;

    #[test]
    fn test_half_adder_works() {
        assert_eq!(half_adder(false, false), (false, false));
        assert_eq!(half_adder(false, true), (true, false));
        assert_eq!(half_adder(true, false), (true, false));
        assert_eq!(half_adder(true, true), (false, true));
    }

    #[test]
    fn test_full_adder_works() {
        assert_eq!(full_adder(false, false, false), (false, false));
        assert_eq!(full_adder(false, false, true), (true, false));
        assert_eq!(full_adder(false, true, false), (true, false));
        assert_eq!(full_adder(false, true, true), (false, true));
        assert_eq!(full_adder(true, false, false), (true, false));
        assert_eq!(full_adder(true, false, true), (false, true));
        assert_eq!(full_adder(true, true, false), (false, true));
        assert_eq!(full_adder(true, true, true), (true, true));
    }

    #[test]
    fn test_add16_works() {
        let val1 = [00,12];
        let val2 = [0,13];
        let sum = [0,25];

        assert_eq!(add16(&bytes_to_boolvec(&val1), &bytes_to_boolvec(&val2)), bytes_to_boolvec(&sum));
    }
}