use std::{fs::File, io::Write};

fn main() {
    println!("Hello, world!");
    let mut v32: Vec<u32> = vec![];
    let mut v: Vec<u8> = vec![];
    for i in 0..100 {
        v32.push(i*100)
    }
    for byte in v32 {
        v.push(((byte &0xFF)).try_into().unwrap());
        v.push((((byte >> 8) &0xFF)).try_into().unwrap());
        v.push((((byte >> 16) &0xFF)).try_into().unwrap());
        v.push((((byte >> 24) &0xFF)).try_into().unwrap());
    }
    let mut file = File::create("generated.ion").unwrap();
    file.write_all(&v).unwrap();
}

// Format: Little-Endian 4-Byte chunks representing u32s