use std::{fs::File, io::Write, time::Instant};
mod pusher;

use pusher::*;

fn main() {
    println!("Creating Buffers");
    let now = Instant::now();

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

    println!("Writing");
    let now2 = Instant::now();

    // Write buffer to file
    let mut file = File::create("generated.ion").unwrap();
    file.write_all(&buffer).unwrap();

    print!("Done. ");
    let time2 = now2.elapsed().as_millis();
    println!("Took {:?} seconds", time2 as f32 / 1000f32);
}
