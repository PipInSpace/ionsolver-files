use std::{fs::File, io::Write};

struct Charge {
    coulomb: f32,
    i: u64,
}

fn main() {
    println!("Writing");
    let mut vcharge: Vec<Charge> = vec![];
    let mut buffer: Vec<u8> = vec![];
    for i in 0..100 {
        vcharge.push(Charge { coulomb: 1.0*i as f32, i: i as u64 });
    }

    let l: u32 = 64;
    let w: u32 = 64;
    let h: u32 = 64;
    // Conversion factors
    let meter_c: f32 = 1.0;
    let kilogram_c: f32 = 1.0;
    let second_c: f32 = 1.0;
    let coulomb_c: f32 = 1.0;

    // Write to buffer
    // Write lenth, width, height
    push32(&mut buffer, l);
    push32(&mut buffer, w);
    push32(&mut buffer, h);
    push32(&mut buffer, meter_c.to_bits());
    push32(&mut buffer, kilogram_c.to_bits());
    push32(&mut buffer, second_c.to_bits());
    push32(&mut buffer, coulomb_c.to_bits());

    // Write all charges
    for charge in vcharge {
        push32(&mut buffer, charge.coulomb.to_bits());
        push64(&mut buffer, charge.i);
    }
    // Write buffer to file
    let mut file = File::create("generated.ion").unwrap();
    file.write_all(&buffer).unwrap();
}

// Push bigger byte sizes to 8-Bit Buffer
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

// Format: Little-Endian 4-Byte chunk representing f32 + 8-Byte Chunk representing u64