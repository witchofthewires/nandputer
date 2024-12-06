use std::iter;

use gates::mux4way16;

use crate::*;

struct HackCtrl {
    zx: bool,
    nx: bool,
    zy: bool, 
    ny: bool,
    f:  bool,
    no: bool,
}

impl HackCtrl {
    fn new(zx: bool, nx: bool, zy: bool, ny: bool, f: bool, no: bool) -> HackCtrl {
        HackCtrl{zx, nx, zy, ny, f, no}
    }
}


struct RISCvCtrl {
    ir: bool,   // R-Type 1, I-Type 0
    al: bool,   // Logic  1, Arith  0
    c: bool, 
    d: bool,
    pn:  bool,  // -Y 1, Y0 0
}

impl RISCvCtrl {
    fn new(ir: bool, al: bool, c: bool, d: bool, pn: bool) -> RISCvCtrl {
        RISCvCtrl{ ir, al, c, d, pn }
    }
}

struct HackOut {
    out: [bool; 16],
    zr: bool,
    ng: bool,
}

impl HackOut {
    fn new(out: [bool; 16], zr: bool, ng: bool) -> HackOut {
        HackOut{out, zr, ng}
    }
}
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
    let (half_sum, half_carr_y) = half_adder(bit1, bit2);
    let (full_sum, half_carry) = half_adder(half_sum, carry);
    (full_sum, gates::xor(half_carr_y, half_carry))
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
fn add16(val1: &[bool; 16], val2: &[bool; 16]) -> [bool; 16] {
    let mut res: [bool; 16] = [false; 16];
    let mut carry_bus: [bool; 17] = [false; 17]; // prevent bus overflow on final bit
    for i in 0..16 {
        (res[i], carry_bus[i+1]) = full_adder(val1[i], val2[i], carry_bus[i]);
    }
    res
}

/// Inc16 - Increment value by 1
/// Inputs: val[16]
/// Outputs: res[16]
/// Function: res=val+1 
/// 
/// Integer 2’s complement addition.
/// Overflow is neither detected nor handled.
fn inc16(val: &[bool; 16]) -> [bool; 16] {
    let one = utils::bytes_to_boollist(&[0,1]);
    add16(&val, &one)
}

/// RISCvALU - ALU core for RISC V CPU
/// 
/// opcode,func3,func7,inst,RISCvCtrl 
/// 0110011,b001,0x00,SLL,10010
/// 0110011,b101,0x20,SRL,11011
/// 0110011,b101,0x00,SRA,11010
/// 0110011,b010,0x00,SLT,10100
/// 0110011,b011,0x00,SLTU,10110
///
/// 0010011,b000,ADDI,0000x
/// 0010011,b100,XORI,0100x
/// 0010011,b110,ORI ,0110x
/// 0010011,b111,ANDI,0111x
/// 0010011,b001,0x00,SLLI,00010
/// 0010011,b101,0x00,SRLI,01010
/// 0010011,b101,0x20,SRAI,01011
/// 0010011,b010,SLTI, 0010x
/// 0010011,b011,SLTIU,0011x
pub fn riscv_alu(val1: &[bool; 16], val2: &[bool; 16], ctrl: &RISCvCtrl) -> [bool; 16] {
    let rs1: [bool; 16] = *val1;
    let mut rs2: [bool; 16] = *val2;
    if ctrl.pn { rs2 = inc16(&gates::not16(&rs2)); }
    
    dbg!(rs1, rs2, ctrl.ir, ctrl.al, ctrl.c, ctrl.d, ctrl.pn);
    let logic_res = match (ctrl.c, ctrl.d) {
        (false,false) => gates::xor16(&rs1, &rs2),
        (true,false)  => gates::or16(&rs1,&rs2),
        (true,true)   => gates::and16(&rs1, &rs2),
        _             => [false; 16]
    };

	gates::mux16(&add16(&rs1, &rs2), &logic_res, ctrl.al)
}

/// Hack_ALU - ALU as specified by nand2tetris
/// Inputs: &[bool; 16], &[bool; 16], &HackCtrl
///            x[16], y[16]: Two 16-bit data inputs
///            zx: Zero the x input
///            nx: Negate the x input
///            zy: Zero the y input
///            ny: Negate the y input
///            f:  Function code: 1 for Add, 0 for And
///            no: Negate the out output
/// }
/// Outputs: HackOut
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
fn hack_alu(val1: &[bool; 16], val2: &[bool; 16], ctrl: &HackCtrl) -> HackOut {

    let zero = utils::bytes_to_boollist(&[0,0]);
    
    let _x = gates::mux16(&val1, &zero, ctrl.zx);
    let x = gates::mux16(&_x, &gates::not16(&_x), ctrl.nx);
    
    let _y = gates::mux16(&val2, &zero, ctrl.zy);
    let y = gates::mux16(&_y, &gates::not16(&_y), ctrl.ny);
    
    let _out = gates::mux16(&gates::and16(&x, &y), &add16(&x, &y), ctrl.f);
    let out = gates::mux16(&_out, &gates::not16(&_out), ctrl.no);

    let zr_1 = gates::or8way(out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7]);
    let zr_2 = gates::or8way(out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[15]);
    let zr = gates::not(gates::or(zr_1, zr_2));

    HackOut::new(out, zr, out[15])
}

#[cfg(test)]
mod tests {
    use utils::bytes_to_boollist;

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

        assert_eq!(add16(&bytes_to_boollist(&val1), &bytes_to_boollist(&val2)), bytes_to_boollist(&sum));
    }

    #[test]
    fn test_inc16_works() {
        let val1 = [00,12];
        let val2 = [0,13];

        assert_eq!(inc16(&bytes_to_boollist(&val1)), bytes_to_boollist(&val2));
    }

    #[test]
    fn test_riscv_alu_works() {
        let val1 = bytes_to_boollist(&[00,12]);
        let val2 = bytes_to_boollist(&[00,13]);
        let sum = bytes_to_boollist(&[00,25]);
        let zero = bytes_to_boollist(&[00,00]);
        let one =  bytes_to_boollist(&[00,1]);
        let neg_one = gates::not16(&zero);

        // 10000 - ADD
        let ctrl = RISCvCtrl::new(true, false, false, false, false);
        let out = riscv_alu(&val1, &val2, &ctrl);
        assert_eq!(utils::boollist_to_bytes(&out), utils::boollist_to_bytes(&sum));

        // 10001 - SUB
        let ctrl = RISCvCtrl::new(true, false, false, false, true);
        let out = riscv_alu(&val1, &val2, &ctrl);
        assert_eq!(utils::boollist_to_bytes(&out), utils::boollist_to_bytes(&neg_one));

        // 11000 - XOR
        let ctrl = RISCvCtrl::new(true, true, false, false, false);
        let out = riscv_alu(&val1, &val2, &ctrl);
        assert_eq!(utils::boollist_to_bytes(&out), utils::boollist_to_bytes(&one));

        // 11100 - OR
        let ctrl = RISCvCtrl::new(true, true, true, false, false);
        let out = riscv_alu(&val1, &val2, &ctrl);
        assert_eq!(utils::boollist_to_bytes(&out), utils::boollist_to_bytes(&val2));

        // 11110 - AND
        let ctrl = RISCvCtrl::new(true, true, true, true, false);
        let out = riscv_alu(&val1, &val2, &ctrl);
        assert_eq!(utils::boollist_to_bytes(&out), utils::boollist_to_bytes(&val1));
    }

    #[test]
    fn test_hack_alu_works() {
        let val1 = bytes_to_boollist(&[00,12]);
        let val2 = bytes_to_boollist(&[00,13]);
        let zero = bytes_to_boollist(&[00,00]);
        let one =  bytes_to_boollist(&[00,1]);
        let neg_one = gates::not16(&zero);

        // 101010: 0
        let ctrl = HackCtrl::new(true, false, true, false, true, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &zero);
        assert_eq!(hackout.zr, true);
        assert_eq!(hackout.ng, false);

        // 111111: 1
        let ctrl = HackCtrl::new(true, true, true, true, true, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &one);
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 111010: -1
        let ctrl = HackCtrl::new(true, true, true, false, true, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &neg_one);
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, true);

        // 001100: x
        let ctrl = HackCtrl::new(false, false, true, true, false, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &val1);
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 110000: y
        let ctrl = HackCtrl::new(true, true, false, false, false, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &val2);
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 001101: !x
        let ctrl = HackCtrl::new(false, false, true, true, false, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &gates::not16(&val1));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, true);

        // 110001: !y
        let ctrl = HackCtrl::new(true, true, false, false, false, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &gates::not16(&val2));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, true);

        // 001111: -x
        let ctrl = HackCtrl::new(false, false, true, true, true, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &inc16(&gates::not16(&val1)));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, true);

        // 110011: -y
        let ctrl = HackCtrl::new(true, true, false, false, true, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &inc16(&gates::not16(&val2)));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, true);

        // 011111: x+1
        let ctrl = HackCtrl::new(false, true, true, true, true, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &inc16(&val1));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 110111: y+1
        let ctrl = HackCtrl::new(true, true, false, true, true, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &inc16(&val2));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 001110: x-1
        let ctrl = HackCtrl::new(false, false, true, true, true, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &add16(&val1, &neg_one));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 110010: y-1
        let ctrl = HackCtrl::new(true, true, false, false, true, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &add16(&val2, &neg_one));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 000010: x+y
        let ctrl = HackCtrl::new(false, false, false, false, true, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &add16(&val1, &val2));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 010011: x-y
        let ctrl = HackCtrl::new( false, true, false, false, true, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &add16(&val1, &inc16(&gates::not16(&val2))));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, true);

        // 000111: y-x
        let ctrl = HackCtrl::new(false, false, false, true, true, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &add16(&val2, &inc16(&gates::not16(&val1))));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);
        
        // 000000: x&y
        let ctrl = HackCtrl::new(false, false, false, false, false, false);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &gates::and16(&val1, &val2));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

        // 010101: x|y
        let ctrl = HackCtrl::new(false, true, false, true, false, true);
        let hackout = hack_alu(&val1, &val2, &ctrl);
        assert_eq!(&hackout.out, &gates::or16(&val1, &val2));
        assert_eq!(hackout.zr, false);
        assert_eq!(hackout.ng, false);

    }
}