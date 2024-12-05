use std::iter;
use crate::*;

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
#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct RAM8 {
    words: [Register; 8],
}

impl RAM8 {
    pub fn new() -> RAM8 {
        RAM8{ words: [Register::new();8] }
    }

    pub fn cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux8way(load, addr[0], addr[1], addr[2]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.words[i].cycle(val, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[0], addr[1], addr[2]))
    }
}

#[derive(Copy, Clone, Debug)]
struct RAM64 {
    ram8s: [RAM8; 8],
}

impl RAM64 {
    fn new() -> RAM64 {
        RAM64{ ram8s: [RAM8::new();8] }
    }

    fn cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux8way(load, addr[0], addr[1], addr[2]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.ram8s[i].cycle(val, addr, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[0], addr[1], addr[2]))
    }
}




#[cfg(test)]
mod tests {
    use utils::bytes_to_boollist;

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
        let input = utils::bytes_to_boollist(&[0xde, 0xad]);
        let zeros = utils::bytes_to_boollist(&[0,0]);

        assert_eq!(register.cycle(&zeros,false),zeros);
        assert_eq!(register.cycle(&zeros,false),zeros);
        assert_eq!(register.cycle(&input,false),zeros);
        assert_eq!(register.cycle(&input,true),zeros);
        assert_eq!(register.cycle(&zeros,false),input);
        assert_eq!(register.cycle(&zeros,false),input);
        assert_eq!(register.cycle(&zeros,true),input);
        assert_eq!(register.cycle(&zeros,false),zeros);
    }

    #[test]
    fn test_ram8_works() {
        let mut ram = RAM8::new();
        let input = utils::bytes_to_boollist(&[0xde, 0xad]);
        let zeros = utils::bytes_to_boollist(&[0,0]);

        // all init to zero
        assert_eq!(ram.cycle(&zeros, &[false,false,false], false), zeros);
        assert_eq!(ram.cycle(&zeros, &[false,false,true], false), zeros);
        assert_eq!(ram.cycle(&zeros, &[false,true,false], false), zeros);
        assert_eq!(ram.cycle(&zeros, &[false,true,true], false), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,false,false], false), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,false,true], false), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,true,false], false), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,true,true], false), zeros);

        // input given but load not set
        assert_eq!(ram.cycle(&input, &[false,false,false], false), zeros);
        assert_eq!(ram.cycle(&input, &[false,false,true], false), zeros);
        assert_eq!(ram.cycle(&input, &[false,true,false], false), zeros);
        assert_eq!(ram.cycle(&input, &[false,true,true], false), zeros);
        assert_eq!(ram.cycle(&input, &[true,false,false], false), zeros);
        assert_eq!(ram.cycle(&input, &[true,false,true], false), zeros);
        assert_eq!(ram.cycle(&input, &[true,true,false], false), zeros);
        assert_eq!(ram.cycle(&input, &[true,true,true], false), zeros);
        
        // input given, load set
        assert_eq!(ram.cycle(&input, &[false,false,false], true), zeros);
        assert_eq!(ram.cycle(&input, &[false,false,true], true), zeros);
        assert_eq!(ram.cycle(&input, &[false,true,false], true), zeros);
        assert_eq!(ram.cycle(&input, &[false,true,true], true), zeros);
        assert_eq!(ram.cycle(&input, &[true,false,false], true), zeros);
        assert_eq!(ram.cycle(&input, &[true,false,true], true), zeros);
        assert_eq!(ram.cycle(&input, &[true,true,false], true), zeros);
        assert_eq!(ram.cycle(&input, &[true,true,true], true), zeros);

        // we now read input as output. zero all
        assert_eq!(ram.cycle(&zeros, &[false,false,false], true), input);
        assert_eq!(ram.cycle(&zeros, &[false,false,true], true), input);
        assert_eq!(ram.cycle(&zeros, &[false,true,false], true), input);
        assert_eq!(ram.cycle(&zeros, &[false,true,true], true), input);
        assert_eq!(ram.cycle(&zeros, &[true,false,false], true), input);
        assert_eq!(ram.cycle(&zeros, &[true,false,true], true), input);
        assert_eq!(ram.cycle(&zeros, &[true,true,false], true), input);
        assert_eq!(ram.cycle(&zeros, &[true,true,true], true), input);
        
        // confirm all output zero
        assert_eq!(ram.cycle(&zeros, &[false,false,false], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[false,false,true], true), zeros);
        assert_eq!(ram.cycle(&input, &[false,true,false], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[false,true,true], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,false,false], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,false,true], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,true,false], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,true,true], true), zeros);

        // confirm only word[2] written
        assert_eq!(ram.cycle(&zeros, &[false,false,false], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[false,false,true], true), zeros);
        assert_eq!(ram.cycle(&input, &[false,true,false], true), input);
        let mut three = utils::bytes_to_boollist(&[0,3]);
        three.reverse();
        assert_eq!(ram.cycle(&zeros, &three, true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,false,false], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,false,true], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,true,false], true), zeros);
        assert_eq!(ram.cycle(&zeros, &[true,true,true], true), zeros);

    }
}