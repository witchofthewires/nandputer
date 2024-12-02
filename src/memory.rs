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

    fn cycle(&mut self, bit: bool) {
        self.bit = bit;
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

    fn read(&self) -> bool {
        self.dff.read()
    }

    fn write(&mut self, val: bool, load: bool) {
        self.dff.cycle(gates::mux(self.read(), val, load));
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

    fn write_bytes(&mut self, bytes: &[u8], load: bool) {
        let bits = gates::bytes_to_boollist(bytes);
        self.write(&bits, load);
    }

    fn write(&mut self, boollist: &[bool], load: bool) {
        if !load { return }
        for i in 0..16 {
            if boollist[i] { self.bits[i].write(true, true); }
        }
    }

    fn read(&self) -> [bool; 16] {
        self.bits.map(|b| b.read())
    }

    fn read_as_bytes(&self) -> [u8; 2] {
        gates::boollist_to_bytes(&self.read())
    }
}

struct RAM8 {
    words: [Register; 8],
}

impl RAM8 {
    fn new() -> RAM8 {
        RAM8{ words: [Register::new();8] }
    }

    fn read_word(&self, addr: [bool; 3]) -> Register {
        Register::new()
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
        let mut bit = BitRegister::new();
        assert_eq!(bit.read(), false);
        bit.write(true, false);
        assert_eq!(bit.read(), false);
        bit.write(true, true);
        assert_eq!(bit.read(), true);
        bit.write(false, false);
        assert_eq!(bit.read(), true);
        bit.write(false, true);
        assert_eq!(bit.read(), false);

    }

    #[test]
    fn test_register_works() {
        let mut register = Register::new();
        let bytes = register.read_as_bytes();
        assert_eq!(bytes[0],0);
        assert_eq!(bytes[1],0);
        
        register.write_bytes(&[0xde as u8, 0xad as u8], false);
        let bytes = register.read_as_bytes();
        assert_eq!(bytes[0],0);
        assert_eq!(bytes[1],0);
        
        register.write_bytes(&[0xde as u8, 0xad as u8], true);
        let bytes = register.read_as_bytes();
        assert_eq!(bytes[0],0xde);
        assert_eq!(bytes[1],0xad);
    }
}