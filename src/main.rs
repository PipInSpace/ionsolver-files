use std::{fs::File, io::Write};

struct Charge {
    coulomb: f32,
    i: u64,
}

struct Wall {
    solid: bool,
}

fn main() {
    println!("Writing");
    let mut vcharge: Vec<Charge> = vec![];
    let mut vwalls: Vec<Wall> = vec![];
    let mut buffer: Vec<u8> = vec![];
    
    // Lenght, Width, Height (x, y, z)
    let l: u32 = 512;
    let w: u32 = 512;
    let h: u32 = 512;
    let n: u32 = l*w*h;
    // Conversion factors
    let meter_c: f32 = 1.0;
    let kilogram_c: f32 = 1.0;
    let second_c: f32 = 1.0;
    let coulomb_c: f32 = 1.0;
    // Fill Vectors
    for _ in 0..n {
        vwalls.push(Wall {solid: true});
    }
    for i in 0..100 {
        vcharge.push(Charge { coulomb: 1.0*i as f32, i: i as u64 });
    }
    

    // Write to buffer
    pushname(&mut buffer);
    // Write lenth, width, height
    push32(&mut buffer, l);
    push32(&mut buffer, w);
    push32(&mut buffer, h);
    push32(&mut buffer, meter_c.to_bits());
    push32(&mut buffer, kilogram_c.to_bits());
    push32(&mut buffer, second_c.to_bits());
    push32(&mut buffer, coulomb_c.to_bits());

    // Write all Walls
    for charge in vwalls.chunks(8) {
        pushwalls(&mut buffer, charge)
    }
    // Write all charges
    for charge in vcharge {
        push32(&mut buffer, charge.coulomb.to_bits());
        push64(&mut buffer, charge.i);
    }
    // Write buffer to file
    let mut file = File::create("generated.ion").unwrap();
    file.write_all(&buffer).unwrap();
}

// Push different byte sizes to 8-Bit Buffer
fn push32(buffer: &mut Vec<u8>, x: u32) {
    for i in 0..4 {
        buffer.push((((x >> i*8) &0xFF)).try_into().unwrap());
    }
}

fn push64(buffer: &mut Vec<u8>, x: u64) {
    for i in 0..8 {
        buffer.push((((x >> i*8) &0xFF)).try_into().unwrap());
    }
}

fn pushwalls(buffer: &mut Vec<u8>, x: &[Wall]) {
    // Pushes a slice of walls into one byte
    // smallest index in most significant bit.
    // If index not divisible by 8, last element contains 0s for non-existant indecies
    let mut byte: u8 = 0;
    for i in 0..x.len() {
        byte += (x[i].solid as u8) << 7-i;
    }
    buffer.push(byte)
}

fn pushname(buffer: &mut Vec<u8>) {
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