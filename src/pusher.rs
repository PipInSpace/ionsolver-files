#[derive(Clone, Copy)]
pub struct Charge {
    pub coulomb: f32,
    pub i: u64,
}

#[derive(Clone, Copy)]
pub struct Wall {
    pub solid: bool,
}

/// Push different byte sizes to 8-Bit Buffer
pub trait ByteBuffer {
    fn push32(&mut self, x: u32);
    fn push64(&mut self, x: u64);
    fn pushwalls(&mut self, x: &[Wall]);
    fn pushname(&mut self);
}

impl ByteBuffer for Vec<u8> {
    /// Pushes the human-readable fileheader
    fn pushname(&mut self) {
        self.push(b'I');
        self.push(b'o');
        self.push(b'n');
        self.push(b'S');
        self.push(b'o');
        self.push(b'l');
        self.push(b'v');
        self.push(b'e');
        self.push(b'r');
        self.push(b' ');
        self.push(b's');
        self.push(b'e');
        self.push(b't');
        self.push(b'u');
        self.push(b'p');
        self.push(b'\n');
    }
    fn push32(&mut self, x: u32) {
        for i in 0..4 {
            self.push(((x >> (i*8)) &0xFF).try_into().unwrap());
        }
    }
    fn push64(&mut self, x: u64) {
        for i in 0..8 {
            self.push(((x >> (i*8)) &0xFF).try_into().unwrap());
        }
    }
    fn pushwalls(&mut self, walls: &[Wall]) {
        // Pushes a slice of walls into one byte
        // smallest index in most significant bit.
        // If index not divisible by 8, last element contains 0s for non-existant indecies
        let mut byte: u8 = 0;
        for (i, wall) in walls.iter().enumerate() {
            byte += (wall.solid as u8) << (7-i);
        }
        self.push(byte)
    }
}


