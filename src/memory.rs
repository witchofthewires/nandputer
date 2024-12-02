use std::iter;
use crate::{gates,adder};

// like nand2tetris we will use the D Flip-Flop as the atomic unit of sequential logic
// in physics DFFs are implemented via feedback between NAND gates
// simulating that would be much more difficult at this stage, instead implement DFF in Rust
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
}