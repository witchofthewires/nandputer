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

pub fn gen_memaddr(val: u16) -> [bool; 16] {
    let byte1: u8 = (val & 0xFF) as u8;
    let byte2: u8 = ((val >> 8) & 0xFF) as u8;
    let mut addr_bits = bytes_to_boollist(&[byte2,byte1]);
    //addr_bits.reverse();
    addr_bits
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

#[cfg(test)]
mod tests {
    use super::*;
 
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

    #[test]
    fn test_gen_memaddr_works() {
        let mut output = [false; 16];
        output[10] = true;
        assert_eq!(gen_memaddr(1024), output);
    }
}