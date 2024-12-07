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

    fn clk_cycle(&mut self, bit: bool) -> bool {
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

    fn read(&self) -> bool {
        self.dff.read()
    }

    fn clk_cycle(&mut self, val: bool, load: bool) -> bool {
        self.dff.clk_cycle(gates::mux(self.dff.read(), val, load))
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
pub struct Register {
    bits: [BitRegister; 16],
}

impl Register {
    fn new() -> Register {
        Register { bits: [BitRegister::new(); 16] }
    }

    fn clk_cycle(&mut self, val: &[bool], load: bool) -> [bool; 16] {
        let mut res = [false; 16];
        for i in 0..16 {
            res[i] = self.bits[i].clk_cycle(val[i], load);
        }
        res
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reading = self.bits.map(|x| x.read());
        for byte in utils::boollist_to_bytes(&reading) { write!(f, "{:02x}", byte)?; }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Register32 {
    bits: [BitRegister; 32],
}

impl Register32 {
    fn new() -> Register32 {
        Register32 { bits: [BitRegister::new(); 32] }
    }

    fn clk_cycle(&mut self, val: &[bool], load: bool) -> [bool; 32] {
        let mut res = [false; 32];
        for i in 0..32 {
            res[i] = self.bits[i].clk_cycle(val[i], load);
        }
        res
    }
}

impl fmt::Display for Register32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reading = self.bits.map(|x| x.read());
        for byte in utils::boollist_to_bytes(&reading) { write!(f, "{:02x}", byte)?; }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RAM8 {
    words: [Register; 8],
    index: usize,
}

impl RAM8 {
    pub fn new() -> RAM8 {
        RAM8{ words: [Register::new();8], index: 0 }
    }

    pub fn clk_cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux8way(load, addr[2], addr[1], addr[0]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.words[i].clk_cycle(val, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[2], addr[1], addr[0]))
    }
}

impl Iterator for RAM8 {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 8 { return None };
        let res = self.words[self.index];
        self.index += 1;
        Some(res)
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
    blocks: [RAM8; 8],
    index: usize,
}

impl RAM64 {
    pub fn new() -> RAM64 {
        RAM64{ blocks: [RAM8::new();8], index: 0 }
    }

    pub fn clk_cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux8way(load, addr[5], addr[4], addr[3]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.blocks[i].clk_cycle(val, &addr, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[5], addr[4], addr[3]))
    }
}

impl Iterator for RAM64 {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 8 { return None };
        let res = match self.blocks[self.index].next() {
            Some(val) => Some(val),
            None => {
                self.index += 1;
                self.next()
            }
        };
        res
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RAM512 {
    blocks: [RAM64; 8],
}

impl RAM512 {
    pub fn new() -> RAM512 {
        RAM512{ blocks: [RAM64::new();8] }
    }

    pub fn clk_cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux8way(load, addr[8], addr[7], addr[6]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.blocks[i].clk_cycle(val, &addr, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[8], addr[7], addr[6]))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RAM4096 {
    blocks: [RAM512; 8],
}

impl RAM4096 {
    pub fn new() -> RAM4096 {
        RAM4096{ blocks: [RAM512::new();8] }
    }

    pub fn clk_cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux8way(load, addr[11], addr[10], addr[9]);
        let mut res = [[false; 16]; 8];
        for i in 0..8 {
            res[i] = self.blocks[i].clk_cycle(val, &addr, load_bits[i]);
        }
        gates::mux8way16(&res, (addr[11], addr[10], addr[9]))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RAM16384 {
    blocks: [RAM4096; 4],
}

impl RAM16384 {
    pub fn new() -> RAM16384 {
        RAM16384{ blocks: [RAM4096::new();4] }
    }

    pub fn clk_cycle(&mut self, val: &[bool], addr: &[bool], load: bool) -> [bool; 16] {
        let load_bits = gates::dmux4way(load, addr[13], addr[12]);
        let mut res = [[false; 16]; 4];
        for i in 0..4 {
            res[i] = self.blocks[i].clk_cycle(val, &addr, load_bits[i]);
        }
        gates::mux4way16(&res, (addr[13], addr[12]))
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
        dff.clk_cycle(true);
        assert_eq!(dff.read(), true);
        dff.clk_cycle(false);
        assert_eq!(dff.read(), false);
    }

    #[test]
    fn test_bit_register_works() {
        let mut bit = BitRegister::new(); // t=0
        assert_eq!(bit.clk_cycle(false,false), false);     // t=1
        assert_eq!(bit.clk_cycle(true,false), false);      // t=2...
        assert_eq!(bit.clk_cycle(false,false), false);
        assert_eq!(bit.clk_cycle(true,true), false);
        assert_eq!(bit.clk_cycle(false,false), true);
        assert_eq!(bit.clk_cycle(false,false), true);
        assert_eq!(bit.clk_cycle(false,true), true);
        assert_eq!(bit.clk_cycle(false,false), false);
        assert_eq!(bit.clk_cycle(false,false), false);
    }

    #[test]
    fn test_register_works() {
        let mut register = Register::new();
        let input = utils::bytes_to_boollist(&[0xde, 0xad]);
        let zeros = utils::bytes_to_boollist(&[0,0]);

        assert_eq!(register.clk_cycle(&zeros,false),zeros);
        assert_eq!(register.clk_cycle(&zeros,false),zeros);
        assert_eq!(register.clk_cycle(&input,false),zeros);
        assert_eq!(register.clk_cycle(&input,true),zeros);
        assert_eq!(register.clk_cycle(&zeros,false),input);
        assert_eq!(register.clk_cycle(&zeros,false),input);
        assert_eq!(register.clk_cycle(&zeros,true),input);
        assert_eq!(register.clk_cycle(&zeros,false),zeros);
    }

    #[test]
    fn test_ram8_works() {
        let mut ram = RAM8::new();
        let input = utils::bytes_to_boollist(&[0xde, 0xad]);
        let zeros = utils::bytes_to_boollist(&[0,0]);

        // all init to zero
        for i in 0..8 { assert_eq!(ram.clk_cycle(&zeros, &utils::gen_memaddr(i), false), zeros); }

        // input given but load not set
        for i in 0..8 { assert_eq!(ram.clk_cycle(&input, &utils::gen_memaddr(i), false), zeros); }
        
        // input given, load set
        for i in 0..8 { assert_eq!(ram.clk_cycle(&input, &utils::gen_memaddr(i), true), zeros); }

        // we now read input as output. zero all
        for i in 0..8 { assert_eq!(ram.clk_cycle(&zeros, &utils::gen_memaddr(i), true), input); }
        
        // confirm all output zero
        for i in 0..8 { assert_eq!(ram.clk_cycle(&zeros, &utils::gen_memaddr(i), true), zeros); }
        assert_eq!(ram.clk_cycle(&input, &[false,true,false], true), zeros);

        // confirm only word[2] written
        for i in 0..8 { 
            if i == 2 { assert_eq!(ram.clk_cycle(&zeros, &utils::gen_memaddr(i), true), input); }
            else { assert_eq!(ram.clk_cycle(&zeros, &utils::gen_memaddr(i), true), zeros); }
        }
    }

    #[test]
    fn test_ram64_works() {
        let size: u16 = 64;
        let mut ram = RAM64::new();
        let zeros = utils::bytes_to_boollist(&[0,0]);

        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(i), false);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(i), true);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(i), true);
            dbg!(i);
            assert_eq!(reading, utils::bytes_to_boollist(&[0,i as u8]));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(i), false);
            dbg!(i);
            assert_eq!(reading, zeros);
        }
    }

    #[test]
    fn test_ram512_works() {
        let size: u16 = 8;
        let mut ram = RAM512::new();
        let addr1 = 137;
        let zeros = utils::bytes_to_boollist(&[0,0]);

        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(addr1-(size/2)+i), false);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(addr1-(size/2)+i), true);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(addr1-(size/2)+i), true);
            dbg!(i);
            assert_eq!(reading, utils::bytes_to_boollist(&[0,i as u8]));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(addr1-(size/2)+i), false);
            dbg!(i);
            assert_eq!(reading, zeros);
        }
    }

    #[test]
    fn test_ram4096_works() {
        let size: u16 = 8;
        let mut ram = RAM4096::new();
        let addr1 = 3913;
        let zeros = utils::bytes_to_boollist(&[0,0]);

        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(addr1-(size/2)+i), false);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(addr1-(size/2)+i), true);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(addr1-(size/2)+i), true);
            dbg!(i);
            assert_eq!(reading, utils::bytes_to_boollist(&[0,i as u8]));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(addr1-(size/2)+i), false);
            dbg!(i);
            assert_eq!(reading, zeros);
        }
    }

    // TODO this test is 8x slower than test_ram4096_works
    // figure out why and you could probably drastically improve perf of rams
    #[test]
    fn test_ram16384_works() {
        let size: u16 = 4;
        let mut ram = RAM16384::new();
        let addr1 = 12130;
        let zeros = utils::bytes_to_boollist(&[0,0]);

        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(i+addr1-(size/2)+size), false);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&utils::bytes_to_boollist(&[0,i as u8]), &utils::gen_memaddr(i+addr1-(size/2)+size), true);
            dbg!(i);
            assert_eq!(utils::boollist_to_bytes(&reading), utils::boollist_to_bytes(&zeros));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(i+addr1-(size/2)+size), true);
            dbg!(i);
            assert_eq!(reading, utils::bytes_to_boollist(&[0,i as u8]));
        }
        for i in 0..size {
            let reading = ram.clk_cycle(&zeros, &utils::gen_memaddr(i+addr1-(size/2)+size), false);
            dbg!(i);
            assert_eq!(reading, zeros);
        }
    }
}
