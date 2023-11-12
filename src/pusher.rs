#[derive(Clone, Copy)]
pub struct Charge {
    pub coulomb: f32,
    pub i: u64,
}

#[derive(Clone, Copy)]
pub struct Wall {
    pub solid: bool,
}


// Push different byte sizes to 8-Bit Buffer

pub fn push32(buffer: &mut Vec<u8>, x: u32) {
    for i in 0..4 {
        buffer.push(((x >> (i*8)) &0xFF).try_into().unwrap());
    }
}

pub fn push64(buffer: &mut Vec<u8>, x: u64) {
    for i in 0..8 {
        buffer.push(((x >> (i*8)) &0xFF).try_into().unwrap());
    }
}

pub fn pushwalls(buffer: &mut Vec<u8>, walls: &[Wall]) {
    // Pushes a slice of walls into one byte
    // smallest index in most significant bit.
    // If index not divisible by 8, last element contains 0s for non-existant indecies
    let mut byte: u8 = 0;
    for (i, wall) in walls.iter().enumerate() {
        byte += (wall.solid as u8) << (7-i);
    }
    buffer.push(byte)
}

pub fn pushname(buffer: &mut Vec<u8>) {
    // Pushes the human-readable fileheader
    buffer.push(b'I');
    buffer.push(b'o');
    buffer.push(b'n');
    buffer.push(b'S');
    buffer.push(b'o');
    buffer.push(b'l');
    buffer.push(b'v');
    buffer.push(b'e');
    buffer.push(b'r');
    buffer.push(b' ');
    buffer.push(b's');
    buffer.push(b'e');
    buffer.push(b't');
    buffer.push(b'u');
    buffer.push(b'p');
    buffer.push(b'\n');
}
