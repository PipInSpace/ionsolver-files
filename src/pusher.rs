pub struct Charge {
    pub coulomb: f32,
    pub i: u64,
}

pub struct Wall {
    pub solid: bool,
}


// Push different byte sizes to 8-Bit Buffer

pub fn push32(buffer: &mut Vec<u8>, x: u32) {
    for i in 0..4 {
        buffer.push((((x >> i*8) &0xFF)).try_into().unwrap());
    }
}

pub fn push64(buffer: &mut Vec<u8>, x: u64) {
    for i in 0..8 {
        buffer.push((((x >> i*8) &0xFF)).try_into().unwrap());
    }
}

pub fn pushwalls(buffer: &mut Vec<u8>, x: &[Wall]) {
    // Pushes a slice of walls into one byte
    // smallest index in most significant bit.
    // If index not divisible by 8, last element contains 0s for non-existant indecies
    let mut byte: u8 = 0;
    for i in 0..x.len() {
        byte += (x[i].solid as u8) << 7-i;
    }
    buffer.push(byte)
}

pub fn pushname(buffer: &mut Vec<u8>) {
    // Pushes the human-readable fileheader
    buffer.push('I' as u8);
    buffer.push('o' as u8);
    buffer.push('n' as u8);
    buffer.push('S' as u8);
    buffer.push('o' as u8);
    buffer.push('l' as u8);
    buffer.push('v' as u8);
    buffer.push('e' as u8);
    buffer.push('r' as u8);
    buffer.push(' ' as u8);
    buffer.push('s' as u8);
    buffer.push('e' as u8);
    buffer.push('t' as u8);
    buffer.push('u' as u8);
    buffer.push('p' as u8);
    buffer.push('\n' as u8);
}
