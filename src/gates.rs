use std::iter;

pub fn nand(val1: bool, val2: bool) -> bool {
    !(val1 & val2)
}

pub fn not(val: bool) -> bool {
    nand(val, val)
}

pub fn not16(val: &[bool; 16]) -> [bool; 16] {
    let mut res = [false; 16];
    for i in 0..16 {
        res[i] = not(val[i]);
    }
    res
}

pub fn and(val1: bool, val2: bool) -> bool {
    not(nand(val1, val2))
}

pub fn and16(val1: &[bool; 16], val2: &[bool; 16]) -> [bool; 16] { 
    let mut res = [false; 16];
    for i in 0..16 {
        res[i] = and(val1[i], val2[i]);
    }
    res
}

pub fn or(val1: bool, val2: bool) -> bool {
    nand(not(val1), not(val2))
}

pub fn or16(val1: &[bool; 16], val2: &[bool; 16]) -> [bool; 16] { 
    let mut res = [false; 16];
    for i in 0..16 {
        res[i] = or(val1[i], val2[i]);
    }
    res
}

pub fn or8way(val1: bool, val2: bool, val3: bool, val4: bool, val5: bool, val6: bool, val7: bool, val8: bool) -> bool {
    or(or(or(val1, val2), or(val3, val4)),or(or(val5, val6), or(val7, val8)))
}

pub fn nor(val1: bool, val2: bool) -> bool {
    not(or(val1, val2))
}

pub fn xor(val1: bool, val2: bool) -> bool {
    and(or(val1, val2), nand(val1, val2))
}

pub fn mux(val1: bool, val2: bool, sel: bool) -> bool {
    or(and(val1, not(sel)), and(val2, sel))    
}

pub fn mux16(val1: &[bool; 16], val2: &[bool; 16], sel: bool) -> [bool; 16] { 
    let mut res = [false; 16];
    for i in 0..16 {
        res[i] = mux(val1[i], val2[i], sel);
    }
    res
}

pub fn mux4way16(val1: &[bool; 16], val2: &[bool; 16], val3: &[bool; 16], val4: &[bool; 16], sel: (bool, bool)) -> [bool; 16] { 
    mux16(&mux16(val1, val3, sel.0), 
        &mux16(val2, val4, sel.0), 
        sel.1)
}

pub fn mux8way16(val1: &[bool; 16], 
            val2: &[bool; 16], 
            val3: &[bool; 16], 
            val4: &[bool; 16], 
            val5: &[bool; 16], 
            val6: &[bool; 16], 
            val7: &[bool; 16], 
            val8: &[bool; 16], 
            sel: (bool, bool, bool)) -> [bool; 16] { 
    mux16(&mux4way16(val1, val2, val3, val4, (sel.1, sel.2)), 
        &mux4way16(val5, val6, val7, val8, (sel.1, sel.2)), 
        sel.0)
}

pub fn dmux(val: bool, sel: bool) -> (bool, bool) {
    (and(val, not(sel)), 
    and(val, sel))
}

pub fn dmux4way(val: bool, sel1: bool, sel2: bool) -> (bool, bool, bool, bool) {
    (and(val, and(not(sel1), not(sel2))), 
    and(val, and(not(sel1), sel2)),
    and(val, and(sel1, not(sel2))),
    and(val, and(sel1, sel2)))

}

pub fn dmux8way(val: bool, sel1: bool, sel2: bool, sel3: bool) -> [bool; 8] {
    [and(val, and(and(not(sel1), not(sel2)), not(sel3))), 
    and(val, and(and(not(sel1), not(sel2)), sel3)), 
    and(val, and(and(not(sel1), sel2), not(sel3))), 
    and(val, and(and(not(sel1), sel2), sel3)), 
    and(val, and(and(sel1, not(sel2)), not(sel3))), 
    and(val, and(and(sel1, not(sel2)), sel3)), 
    and(val, and(and(sel1, sel2), not(sel3))), 
    and(val, and(and(sel1, sel2), sel3))] 
}

// TODO this should be factored out
pub fn bytes_to_boolvec(bytes: &[u8]) -> Vec<bool> {
    let mut boolvec = Vec::new();
    for byte in bytes {
        for i in 0..8 {
            let val = (byte >> (7-i)) & 1;
            if val == 1 { boolvec.push(true); }
            else { boolvec.push(false); }
        }
    }
    boolvec
}


// TODO these should be refactored into utils.rs
pub fn bytes_to_boollist(bytes: &[u8]) -> [bool; 16] {
    let mut boollist = [false; 16];
    let mut total = 0;
    for byte in bytes {
        for i in 0..8 {
            let val = (byte >> (7-i)) & 1;
            if val == 1 { boollist[15 - total] = true; }
            else { boollist[15 - total] = false; }
            total += 1;
        }
        if total == 16 { break } // cap at 16 bits for now
    }
    boollist
}

pub fn boollist_to_bytes(boollist: &[bool]) -> [u8; 2] {
    let base: u8 = 2;
    let mut val1: u8 = 0;
    let mut val2: u8 = 0;
    for i in 0..8 {
        if boollist[i] { val1 += base.pow(i.try_into().expect("Failed to parse first byte from boollist")); }
    }
    for i in 8..16 {
        if boollist[i] { val2 += base.pow((i-8).try_into().expect("Failed to parse second byte from boollist")); }
    }
    [val2, val1] // little endian 
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
    fn test_dmux4way_works() {
        assert_eq!(dmux4way(false, false, false), (false, false, false, false));
        assert_eq!(dmux4way(false, false, true), (false, false, false, false));
        assert_eq!(dmux4way(false, true, false), (false, false, false, false));
        assert_eq!(dmux4way(false, true, true), (false, false, false, false));
        assert_eq!(dmux4way(true, false, false), (true, false, false, false));
        assert_eq!(dmux4way(true, false, true), (false, true, false, false));
        assert_eq!(dmux4way(true, true, false), (false, false, true, false));
        assert_eq!(dmux4way(true, true, true), (false, false, false, true));
    }

    #[test]
    fn test_dmux8way_works() {
        assert_eq!(dmux8way(false, false, false, false), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(false, false, false, true), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(false, false, true, false), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(false, false, true, true), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(false, true, false, false), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(false, true, false, true), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(false, true, true, false), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(false, true, true, true), [false, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(true, false, false, false), [true, false, false, false, false, false, false, false]);
        assert_eq!(dmux8way(true, false, false, true), [false, true, false, false, false, false, false, false]);
        assert_eq!(dmux8way(true, false, true, false), [false, false, true, false, false, false, false, false]);
        assert_eq!(dmux8way(true, false, true, true), [false, false, false, true, false, false, false, false]);
        assert_eq!(dmux8way(true, true, false, false), [false, false, false, false, true, false, false, false]);
        assert_eq!(dmux8way(true, true, false, true), [false, false, false, false, false, true, false, false]);
        assert_eq!(dmux8way(true, true, true, false), [false, false, false, false, false, false, true, false]);
        assert_eq!(dmux8way(true, true, true, true), [false, false, false, false, false, false, false, true]);

    }

    #[test]
    fn test_not16_works() {
        let val = [false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let res = [true, false, true, false, false, false, true, false, true, true, true, false, false, true, false, false];
        assert_eq!(not16(&val), res);
    }

    #[test]
    fn test_and16_works() {
        let val1 = [false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let val2 = [false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, false];
        let res =  [false, false, false, true, false, true, false, true, false, false, false, true, true, false, true, false];
        assert_eq!(and16(&val1, &val2), res);
    }

    #[test]
    fn test_or16_works() {
        let val1 = [false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let val2 = [false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, false];
        let res =  [false, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true];
        assert_eq!(or16(&val1, &val2), res);
    }

    #[test]
    fn test_mux16_works() {
        let val1 = [false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let val2 =[false, false, true, true, false, true, true, true, true, false, false, true, true, true, true, false];
        assert_eq!(mux16(&val1, &val2, false), val1);
        assert_eq!(mux16(&val1, &val2, true), val2);
    }

    #[test]
    fn test_mux4way16_works() {
        let val1 = bytes_to_boollist(&[0x5d, 0x1b]);
        let val2 = bytes_to_boollist(&[0x37, 0x9e]);
        let val3 = bytes_to_boollist(&[0x9f, 0x66]);
        let val4 = bytes_to_boollist(&[0x54, 0xd3]);
        assert_eq!(mux4way16(&val1, &val2, &val3, &val4, (false, false)), val1);
        assert_eq!(mux4way16(&val1, &val2, &val3, &val4, (false, true)), val2);
        assert_eq!(mux4way16(&val1, &val2, &val3, &val4, (true, false)), val3);
        assert_eq!(mux4way16(&val1, &val2, &val3, &val4, (true, true)), val4);
    }

    #[test]
    fn test_mux8way16_works() {
        let val1 = bytes_to_boollist(&[0x5d, 0x1b]);
        let val2 = bytes_to_boollist(&[0x37, 0x9e]);
        let val3 = bytes_to_boollist(&[0x9f, 0x66]);
        let val4 = bytes_to_boollist(&[0x54, 0xd3]);
        let val5 = bytes_to_boollist(&[0x12, 0x34]);
        let val6 = bytes_to_boollist(&[0x56, 0x78]);
        let val7 = bytes_to_boollist(&[0x90, 0xab]);
        let val8 = bytes_to_boollist(&[0xcd, 0xef]);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (false, false, false)), val1);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (false, false, true)), val2);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (false, true, false)), val3);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (false, true, true)), val4);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (true, false, false)), val5);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (true, false, true)), val6);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (true, true, false)), val7);
        assert_eq!(mux8way16(&val1, &val2, &val3, &val4, &val5, &val6, &val7, &val8, (true, true, true)), val8);
    }
 
    #[test]
    fn test_or8way_works() {
        for byte in 0..=255 {
            let mut expected: bool = true;
            if (byte == 0) { expected = false; }
            
            let mut inputs: [bool; 8] = [false; 8];
            for i in 0..8 {
                inputs[i] = (((byte >> i) & 1) == 1);
            }
            assert_eq!(or8way(inputs[0], inputs[1], inputs[2], inputs[3], inputs[4], inputs[5], inputs[6], inputs[7]), expected);
        }
    }

    #[test]
    fn test_bytes_to_boolvec_works() {
        let val = vec![false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        let val_bytes = [0x5d, 0x1b];
        assert_eq!(bytes_to_boolvec(&val_bytes), val);
    }

    #[test]
    fn test_boollist_to_bytes_works() {
        let mut val = [false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        val.reverse(); // little endian
        let val_bytes = [0x5d, 0x1b];
        assert_eq!(boollist_to_bytes(&val), val_bytes);
    }

    #[test]
    fn test_bytes_to_boollist_works() {
        let val_bytes = [0x5d, 0x1b];
        let mut val = [false, true, false, true, true, true, false, true, false, false, false, true, true, false, true, true];
        val.reverse(); // little endian
        assert_eq!(bytes_to_boollist(&val_bytes), val);
    }
}