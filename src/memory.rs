use std::{iter,fmt};
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

impl fmt::Display for BitRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.dff.read() {
            true =>  write!(f, "1"),
            false => write!(f, "0"),
        }
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

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // need to reverse output to big-endian for human viewing
        for bit in self.bits.into_iter().rev() { write!(f, "{}", bit)?; }
        Ok(())
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
        let load_bits = gates::dmux8way(load, addr[2], addr[1], addr[0]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.words[i].cycle(val, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[2], addr[1], addr[0]))
    }
}

impl fmt::Display for RAM8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8 { write!(f, "{}: {}\n", i, self.words[i])?; }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RAM64 {
    ram8s: [RAM8; 8],
}

impl RAM64 {
    pub fn new() -> RAM64 {
        RAM64{ ram8s: [RAM8::new();8] }
    }

    pub fn cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux8way(load, addr[5], addr[4], addr[3]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.ram8s[i].cycle(val, &addr, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[5], addr[4], addr[3]))
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
        for i in 0..8 { assert_eq!(ram.cycle(&zeros, &utils::gen_memaddr(i), false), zeros); }

        // input given but load not set
        for i in 0..8 { assert_eq!(ram.cycle(&input, &utils::gen_memaddr(i), false), zeros); }
        
        // input given, load set
        for i in 0..8 { assert_eq!(ram.cycle(&input, &utils::gen_memaddr(i), true), zeros); }

        // we now read input as output. zero all
        for i in 0..8 { assert_eq!(ram.cycle(&zeros, &utils::gen_memaddr(i), true), input); }
        
        // confirm all output zero
        for i in 0..8 { assert_eq!(ram.cycle(&zeros, &utils::gen_memaddr(i), true), zeros); }
        assert_eq!(ram.cycle(&input, &[false,true,false], true), zeros);

        // confirm only word[2] written
        for i in 0..8 { 
            if i == 2 { assert_eq!(ram.cycle(&zeros, &utils::gen_memaddr(i), true), input); }
            else { assert_eq!(ram.cycle(&zeros, &utils::gen_memaddr(i), true), zeros); }
        }
    }

    #[test]
    fn test_ram64_works() {
        let mut ram = RAM64::new();
        let input = utils::bytes_to_boollist(&[0xde, 0xad]);
        let zeros = utils::bytes_to_boollist(&[0,0]);
        let addr = 7;
        let mut addr_bits = utils::bytes_to_boollist(&[0,addr]);
        addr_bits.reverse();

        for i in 0..64 {
            let reading = ram.cycle(&utils::bytes_to_boollist(&[0,i]), &utils::gen_memaddr(i.into()), false);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..64 {
            let reading = ram.cycle(&utils::bytes_to_boollist(&[0,i]), &utils::gen_memaddr(i.into()), true);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..64 {
            let reading = ram.cycle(&zeros, &utils::gen_memaddr(i), true);
            dbg!(i);
            assert_eq!(reading, utils::bytes_to_boollist(&[0,i as u8]));
        }
        for i in 0..64 {
            let reading = ram.cycle(&zeros, &utils::gen_memaddr(i), false);
            dbg!(i);
            assert_eq!(reading, zeros);
        }
        /* 
        ram.cycle(&input, &addr_bits, true);
        for i in 0..64 {
            let reading = ram.cycle(&input, &utils::gen_memaddr(i), false);
            if i == addr.into() { assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&input)); }
            else { assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros)); }
        }
        ram.cycle(&zeros, &addr_bits, true);
        for i in 0..64 {
            let reading = ram.cycle(&input, &utils::gen_memaddr(i), false);
            let reading = ram.cycle(&input, &addr_bits, false);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }*/
    }
}