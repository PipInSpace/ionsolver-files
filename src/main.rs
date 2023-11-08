use std::{fs::File, io::{Write, self}, time::Instant};
mod pusher;
mod simvalues;

use pusher::*;
use simvalues::SimValues;

fn main() {

    let buffer = encode(512, 512, 512, 1.0, 1.0, 1.0, 1.0);

    println!("Writing");
    let now2 = Instant::now();

    // Write buffer to file
    let mut file = File::create("generated.ion").unwrap();
    file.write_all(&buffer).unwrap();

    print!("Done. ");
    let time2 = now2.elapsed().as_millis();
    println!("Took {:?} seconds", time2 as f32 / 1000f32);
}

fn encode(l : u32, w : u32, h : u32, meter : f32, kilogram: f32, second: f32, coulomb: f32, /* charges: Vec<Charge>, Walls */) -> Vec<u8> {
    println!("Creating Buffers");
    let now = Instant::now();

    let mut vcharge: Vec<Charge> = vec![];
    let mut vwalls: Vec<Wall> = vec![];
    let mut buffer: Vec<u8> = vec![];
    
    // Lenght, Width, Height (x, y, z)
    //let l: u32 = 512;
    //let w: u32 = 512;
    //let h: u32 = 512;
    let n: u32 = l*w*h;
    // Conversion factors
    //let meter_c: f32 = 1.0;
    //let kilogram_c: f32 = 1.0;
    //let second_c: f32 = 1.0;
    //let coulomb_c: f32 = 1.0;
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
    push32(&mut buffer, meter.to_bits());
    push32(&mut buffer, kilogram.to_bits());
    push32(&mut buffer, second.to_bits());
    push32(&mut buffer, coulomb.to_bits());

    // Write all Walls
    for wall in vwalls.chunks(8) {
        pushwalls(&mut buffer, wall)
    }

    push32(&mut buffer, vcharge.len() as u32);
    // Write all charges
    for charge in vcharge {
        push32(&mut buffer, charge.coulomb.to_bits());
        push64(&mut buffer, charge.i);
    }

    print!("Done. ");
    let time = now.elapsed().as_millis();
    println!("Took {:?} seconds", time as f32 / 1000f32);

    return buffer;
}

fn decode(buffer: Vec<u8>) -> Result<SimValues, String> {
    let mut pos: usize = 0;
    let mut header = "".to_owned();
    for _ in 0..15 {
        header.push(buffer[pos] as char);
        pos += 1;
    }
    if header != "IonSolver setup" {
        return Err("Invalid format!".to_owned());
    }


    // Values
    let l = to_u32(get_next_chunk(&buffer, pos));
    let w = to_u32(get_next_chunk(&buffer, pos));
    let h = to_u32(get_next_chunk(&buffer, pos));
    
    let m = to_u32(get_next_chunk(&buffer, pos));
    let kg = to_u32(get_next_chunk(&buffer, pos));
    let s = to_u32(get_next_chunk(&buffer, pos));
    let c = to_u32(get_next_chunk(&buffer, pos));

    // Walls
    let mut walls: Vec<bool>;
    for i in 0..(l*w*h/(4 * 8)) {
        let chunk = get_next_chunk(&buffer, pos);
        for j in 0..4 {
            let b = chunk[j];
            for bit in 0..8 {
                walls.push((b >> bit) & (1 as u8) == 1)
            }
        }
    }

    // Charges
    let len = to_u32(get_next_chunk(&buffer, pos));

    let mut charges: Vec<Charge>;
    for i in 0..len {
        charges.push(Charge {
            coulomb: to_f32(get_next_chunk(&buffer, pos)),
            pos: 
        })
    }

    return Ok(None);
}

fn get_next_chunk(buffer: &Vec<u8>, mut pos: usize) -> [u8; 4] {
    let mut v: [u8; 4];
    v[0] = buffer[pos];
    v[1] = buffer[pos + 1];
    v[2] = buffer[pos + 2];
    v[3] = buffer[pos + 3];
    pos += 4;

    return v;
}

fn to_u32(v: [u8; 4]) -> u32 {
    return v[0] as u32 + ((v[1] as u32) << 8) + ((v[2] as u32) << 16) + ((v[3] as u32) << 24);
}

fn to_u64(v1: [u8; 4], v2: [u8; 4]) -> u32 {
    //return v[0] as u32 + ((v[1] as u32) << 8) + ((v[2] as u32) << 16) + ((v[3] as u32) << 24);
    return 0;
}

fn to_f32(v: [u8; 4]) -> f32 {
    return f32::from_le_bytes(v);
}
