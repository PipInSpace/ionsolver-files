use std::{fs::File, io::Write, time::Instant};
mod pusher;
mod simvalues;

use pusher::*;
use simvalues::SimValues;

fn main() {

    let buffer = encode(128, 128, 256, 1.0, 1.0, 1.0, 1.0);
    let values = decode(&buffer).unwrap();

    println!("L: {}", values.n_x);
    println!("W: {}", values.n_y);
    println!("H: {}", values.n_z);
    println!("M: {}", values.m);
    println!("Kg: {}", values.kg);
    println!("S: {}", values.s);
    println!("C: {}", values.c);

    println!("Writing");
    let now2 = Instant::now();

    // Write buffer to file
    let mut file = File::create("generated.ion").unwrap();
    file.write_all(&buffer).unwrap();

    print!("Done. ");
    let time2 = now2.elapsed().as_millis();
    println!("Took {:?} seconds", time2 as f32 / 1000f32);
}

fn encode(n_x : u32, n_y : u32, n_z : u32, meter : f32, kilogram: f32, second: f32, coulomb: f32, /* charges: Vec<Charge>, Walls */) -> Vec<u8> {
    println!("Creating Buffers");
    let now = Instant::now();

    let vcharge: Vec<Charge> = vec![];
    let mut buffer: Vec<u8> = vec![];
    
    let n: u32 = n_x*n_y*n_z;
    // Fill Vectors
    let mut vwalls: Vec<Wall> = vec![Wall {solid: false}; n as usize];
    for i in 0..(n_x*n_y*8) {
        vwalls[i as usize] = Wall {solid: true};
        vwalls[(n - 1 - i) as usize] = Wall {solid: true};
    }
    //for i in 0..100 {
    //    vcharge.push(Charge { coulomb: 1.0*i as f32, i: i as u64 });
    //}
    let mut vmagnet: Vec<(u64, [f32; 3])> = vec![];
    for i in 0..243 {
        vmagnet.push((i * 68, [0.0, 0.0, 64000000000.0]));
        vmagnet.push((i * 68 + 2097152 * 2, [0.0, 0.0, 64000000000.0]));
    }
    

    // Write to buffer
    println!("Writing...");
    buffer.pushname();
    // Write lenth, width, height
    buffer.push32(n_x);
    buffer.push32(n_y);
    buffer.push32(n_z);
    buffer.push32(meter.to_bits());
    buffer.push32(kilogram.to_bits());
    buffer.push32(second.to_bits());
    buffer.push32(coulomb.to_bits());

    // Write all Walls
    for wall_chunk in vwalls.chunks(8) {
        buffer.pushwalls(wall_chunk);
    }

    buffer.push32(vcharge.len() as u32);
    // Write all charges
    for charge in vcharge {
        buffer.push32(charge.coulomb.to_bits());
        buffer.push64(charge.i);
    }
    buffer.push32(vmagnet.len() as u32);
    // Write all magnets
    for magnet in vmagnet {
        buffer.push64(magnet.0);
        buffer.push32(magnet.1[0].to_bits());
        buffer.push32(magnet.1[1].to_bits());
        buffer.push32(magnet.1[2].to_bits());
    }

    let time = now.elapsed().as_millis();
    print!("Done. ");
    println!("Took {:?} seconds", time as f32 / 1000f32);

    buffer
}

fn decode(buffer: &[u8]) -> Result<SimValues, String> {
    println!("Decoding Buffers");
    let now = Instant::now();

    let mut pos: usize = 0;
    let mut header = "".to_owned();
    for _ in 0..15 {
        header.push(buffer[pos] as char);
        pos += 1;
    }
    if header != "IonSolver setup" {
        return Err("Invalid format!".to_owned());
    }

    pos += 1; // increment to skip next byte so we are 4 byte aligned again

    // Values
    let l = to_u32(get_next_chunk(buffer, &mut pos));
    let w = to_u32(get_next_chunk(buffer, &mut pos));
    let h = to_u32(get_next_chunk(buffer, &mut pos));
    
    let m = to_f32(get_next_chunk(buffer, &mut pos));
    let kg = to_f32(get_next_chunk(buffer, &mut pos));
    let s = to_f32(get_next_chunk(buffer, &mut pos));
    let c = to_f32(get_next_chunk(buffer, &mut pos));

    // Walls
    let mut walls: Vec<bool> = vec![];
    for _ in 0..(l*w*h/(4 * 8)) {
        let chunk = get_next_chunk(buffer, &mut pos);
        for byte in chunk {
            for bit in 0..8 {
                walls.push((byte >> (7-bit)) & 1_u8 == 1);
            }
        }
    }

    // Charges
    let len = to_u32(get_next_chunk(buffer, &mut pos));
    //println!("Len: {}", len);

    let mut charges: Vec<Charge> = vec![];
    for _ in 0..len {
        let charge = to_f32(get_next_chunk(buffer, &mut pos));
        let i1 = get_next_chunk(buffer, &mut pos);
        let i2 = get_next_chunk(buffer, &mut pos);
        charges.push(Charge {
            coulomb: charge,
            i: to_u64(i1, i2),
        })
    }

    let values = SimValues {
        n_x: l,
        n_y: w,
        n_z: h,
        m,
        kg,
        s,
        c,
        walls,
        charges,
    };

    let time = now.elapsed().as_millis();
    print!("Done. ");
    println!("Took {:?} seconds", time as f32 / 1000f32);

    Ok(values)
}

fn get_next_chunk(buffer: &[u8], pos: &mut usize) -> [u8; 4] {
    let mut v =  [0; 4];
    v[0] = buffer[*pos];
    v[1] = buffer[*pos + 1];
    v[2] = buffer[*pos + 2];
    v[3] = buffer[*pos + 3];
    *pos += 4;

    v
}

fn to_u32(v: [u8; 4]) -> u32 {
    v[0] as u32 + ((v[1] as u32) << 8) + ((v[2] as u32) << 16) + ((v[3] as u32) << 24)
}

fn to_u64(vlow: [u8; 4], vhigh: [u8; 4]) -> u64 {
    //println!("u64: {}", to_u32(vlow) as u64 + ((to_u32(vhigh) as u64) << 32));
    to_u32(vlow) as u64 + ((to_u32(vhigh) as u64) << 32)
}

fn to_f32(v: [u8; 4]) -> f32 {
    //return f32::from_le_bytes([0, 0, 0x80, 0x3f]); // 1.0
    //println!("f32: {} {} {} {}", v[0], v[1], v[2], v[3]);
    //println!("f32: {}", f32::from_le_bytes(v));
    f32::from_le_bytes(v)
}
