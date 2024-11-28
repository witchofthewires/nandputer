use std::iter;

use crate::gates;

/// half_adder - Add two bits
/// Inputs: bit1, bit2
/// Outputs: (sum, carry)
/// Function: sum is LSB of bit1 + bit2, carry is MSB
fn half_adder(bit1: bool, bit2: bool) -> (bool, bool) {
    (gates::xor(bit1, bit2), 
     gates::and(bit1, bit2))
}

/// half_adder - Add two bits and carry input
/// Inputs: bit1, bit2, carry
/// Outputs: (sum, carry_out)
/// Function: sum is LSB of bit1 + bit2 + carry, carry_out is MSB
fn full_adder(bit1: bool, bit2: bool, carry: bool) -> (bool, bool) {
    let (half_sum, half_carry_1) = half_adder(bit1, bit2);
    let (full_sum, half_carry_2) = half_adder(half_sum, carry);
    (full_sum, gates::xor(half_carry_1, half_carry_2))
}

// carry-ripple adder
// TODO replace with carry-lookahead
/// Add16 - Add two 16-bit values
/// Inputs: val1[16], val2[16]
/// Outputs: res[16]
/// Function: res=val1+val2
/// 
/// Integer 2’s complement addition.
/// Overflow is neither detected nor handled.
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

/// Inc16 - Increment value by 1
/// Inputs: val[16]
/// Outputs: res[16]
/// Function: res=val+1 
/// 
/// Integer 2’s complement addition.
/// Overflow is neither detected nor handled.
fn inc16(val: &Vec<bool>) -> Vec<bool> {
    let one = gates::bytes_to_boolvec(&[0,1]);
    add16(&val, &one)
}

/// Hack_ALU - ALU as specified by nand2tetris
/// Inputs {
///            x[16], y[16]: Two 16-bit data inputs
///            zx: Zero the x input
///            nx: Negate the x input
///            zy: Zero the y input
///            ny: Negate the y input
///            f:  Function code: 1 for Add, 0 for And
///            no: Negate the out output
/// }
/// Outputs {
///            out[16]: 16-bit output
///            zr: True iff out=0
///            ng: True iff out<0
/// }
/// Function { 
///            if zx then x = 0, 16-bit zero constant
///            if nx then x = !x, Bit-wise negation
///            if zy then y = 0, 16-bit zero constant
///            if ny then y = !y, Bit-wise negation
///            if f then out = x + y, Integer 2's complement addition
///            else out = x & y, Bit-wise And
///            if no then out = !out, Bit-wise negation
///            if out=0 then zr = 1 else zr = 0, 16-bit eq. comparison
///            if out<0 then ng = 1 else ng = 0, 16-bit neg. comparison
/// }
/// 
/// Overflow is neither detected nor handled.
fn hack_alu(val1: &Vec<bool>, val2: &Vec<bool>, zx: bool, nx: bool, zy: bool, ny: bool, f: bool, no: bool) -> (Vec<bool>, bool, bool) {
    let zero = gates::bytes_to_boolvec(&[0,0]);
    //let just_one = gates::bytes_to_boolvec(&[0,1]);
    //let ones = gates::bytes_to_boolvec(&[0xFF, 0xFF]);
    let x_1 = gates::mux16(&val1, &zero, zx);
    let x_2 = gates::mux16(&x_1, &gates::not16(&x_1), nx);
    let y_1 = gates::mux16(&val2, &zero, zy);
    let y_2 = gates::mux16(&y_1, &gates::not16(&y_1), ny);
    let out_1 = gates::mux16(&gates::and16(&x_2, &y_2), &add16(&x_2, &y_2), f);
    let out_2 = gates::mux16(&out_1, &gates::not16(&out_1), no);
    (out_2, false, false) // TODO implement
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

    #[test]
    fn test_inc16_works() {
        let val1 = [00,12];
        let val2 = [0,13];

        assert_eq!(inc16(&bytes_to_boolvec(&val1)), bytes_to_boolvec(&val2));
    }

    #[test]
    fn test_hack_alu_works() {
        let val1 = bytes_to_boolvec(&[00,12]);
        let val2 = bytes_to_boolvec(&[00,13]);
        let zero = bytes_to_boolvec(&[00,00]);
        let one =  bytes_to_boolvec(&[00,01]);
        let neg_one = gates::not16(&zero);

        // 101010: 0
        let (out, _, _) = hack_alu(&val1, &val2, true, false, true, false, true, false);
        assert_eq!(&out, &zero);

        // 111111: 1
        let (out, _, _) = hack_alu(&val1, &val2, true, true, true, true, true, true);
        assert_eq!(&out, &one);

        // 111010: -1
        let (out, _, _) = hack_alu(&val1, &val2, true, true, true, false, true, false);
        assert_eq!(&out, &neg_one);

        // 001100: x
        let (out, _, _) = hack_alu(&val1, &val2, false, false, true, true, false, false);
        assert_eq!(&out, &val1);

        // 110000: y
        let (out, _, _) = hack_alu(&val1, &val2, true, true, false, false, false, false);
        assert_eq!(&out, &val2);

        // 001101: !x
        let (out, _, _) = hack_alu(&val1, &val2, false, false, true, true, false, true);
        assert_eq!(&out, &gates::not16(&val1));

        // 110001: !y
        let (out, _, _) = hack_alu(&val1, &val2, true, true, false, false, false, true);
        assert_eq!(&out, &gates::not16(&val2));

        // 001111: -x
        let (out, _, _) = hack_alu(&val1, &val2, false, false, true, true, true, true);
        assert_eq!(&out, &inc16(&gates::not16(&val1)));

        // 110011: -y
        let (out, _, _) = hack_alu(&val1, &val2, true, true, false, false, true, true);
        assert_eq!(&out, &inc16(&gates::not16(&val2)));

        // 011111: x+1
        let (out, _, _) = hack_alu(&val1, &val2, false, true, true, true, true, true);
        assert_eq!(&out, &inc16(&val1));

        // 110111: y+1
        let (out, _, _) = hack_alu(&val1, &val2, true, true, false, true, true, true);
        assert_eq!(&out, &inc16(&val2));

        // 001110: x-1
        let (out, _, _) = hack_alu(&val1, &val2, false, false, true, true, true, false);
        assert_eq!(&out, &add16(&val1, &neg_one));

        // 110010: y-1
        let (out, _, _) = hack_alu(&val1, &val2, true, true, false, false, true, false);
        assert_eq!(&out, &add16(&val2, &neg_one));

        // 000010: x+y
        let (out, _, _) = hack_alu(&val1, &val2, false, false, false, false, true, false);
        assert_eq!(&out, &add16(&val1, &val2));

        // 010011: x-y
        let (out, _, _) = hack_alu(&val1, &val2, false, true, false, false, true, true);
        assert_eq!(&out, &add16(&val1, &inc16(&gates::not16(&val2))));

        // 000111: y-x
        let (out, _, _) = hack_alu(&val1, &val2, false, false, false, true, true, true);
        assert_eq!(&out, &add16(&val2, &inc16(&gates::not16(&val1))));
        
        // 000000: x&y
        let (out, _, _) = hack_alu(&val1, &val2, false, false, false, false, false, false);
        assert_eq!(&out, &gates::and16(&val1, &val2));

        // 010101: x|y
        let (out, _, _) = hack_alu(&val1, &val2, false, true, false, true, false, true);
        assert_eq!(&out, &gates::or16(&val1, &val2));

    }
}