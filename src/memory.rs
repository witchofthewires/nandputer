use std::iter;
use crate::{gates,adder};

// like nand2tetris we will use the D Flip-Flop as the atomic unit of sequential logic
// in physics DFFs are implemented via feedback between NAND gates
// simulating that would be much more difficult at this stage, instead implement DFF in Rust
#[derive(Copy, Clone, Debug)]
struct DFF {
    bit: bool
}

// TODO is there a kind of iterator that can be passed values?
impl DFF {
    fn new() -> DFF {
        DFF { bit: false }
    }

    fn read(&self) -> bool {
        self.bit
    }

    fn cycle(&mut self, bit: bool) -> bool {
        let res = self.bit;
        self.bit = bit;
        res
    }
}

#[derive(Copy, Clone, Debug)]
struct BitRegister {
    dff: DFF
}

impl BitRegister {

    fn new() -> BitRegister {
        BitRegister{ dff: DFF::new() }
    }

    fn cycle(&mut self, val: bool, load: bool) -> bool {
        self.dff.cycle(gates::mux(self.dff.read(), val, load))
    }
}

// Chip name: RAMn // n and k are listed below
// Inputs: in[16], address[k], load
// Outputs: out[16]
// Function: out(t)=RAM[address(t)](t)
// If load(t-1) then
// RAM[address(t-1)](t)=in(t-1)
// Comment: "=" is a 16-bit operation.
// The specific RAM chips needed for the Hack platform are:
// Chip name n k
// RAM8 8 3
// RAM64 64 6
// RAM512 512 9
// RAM4K 4096 12
// RAM16K 16384 14
#[derive(Copy, Clone)]
struct Register {
    bits: [BitRegister; 16],
}

impl Register {
    fn new() -> Register {
        Register { bits: [BitRegister::new(); 16] }
    }

    fn cycle(&mut self, val: &[bool], load: bool) -> [bool; 16] {
        let mut res = [false; 16];
        for i in 0..16 {
            res[i] = self.bits[i].cycle(val[i], load);
        }
        res
    }
}

struct RAM8 {
    words: [Register; 8],
}

impl RAM8 {
    fn new() -> RAM8 {
        RAM8{ words: [Register::new();8] }
    }
}

#[cfg(test)]
mod tests {
    use gates::bytes_to_boollist;

    use super::*;

    #[test]
    fn test_dff_works() {
        let mut dff = DFF::new();
        assert_eq!(dff.read(), false);
        dff.cycle(true);
        assert_eq!(dff.read(), true);
        dff.cycle(false);
        assert_eq!(dff.read(), false);
    }

    #[test]
    fn test_bit_register_works() {
        let mut bit = BitRegister::new(); // t=0
        assert_eq!(bit.cycle(false,false), false);     // t=1
        assert_eq!(bit.cycle(true,false), false);      // t=2...
        assert_eq!(bit.cycle(false,false), false);
        assert_eq!(bit.cycle(true,true), false);
        assert_eq!(bit.cycle(false,false), true);
        assert_eq!(bit.cycle(false,false), true);
        assert_eq!(bit.cycle(false,true), true);
        assert_eq!(bit.cycle(false,false), false);
        assert_eq!(bit.cycle(false,false), false);
    }

    #[test]
    fn test_register_works() {
        let mut register = Register::new();
        let input = gates::bytes_to_boollist(&[0xde, 0xad]);
        let zeros = gates::bytes_to_boollist(&[0,0]);

        assert_eq!(register.cycle(&zeros,false),zeros);
        assert_eq!(register.cycle(&zeros,false),zeros);
        assert_eq!(register.cycle(&input,false),zeros);
        assert_eq!(register.cycle(&input,true),zeros);
        assert_eq!(register.cycle(&zeros,false),input);
        assert_eq!(register.cycle(&zeros,false),input);
        assert_eq!(register.cycle(&zeros,true),input);
        assert_eq!(register.cycle(&zeros,false),zeros);
    }
}