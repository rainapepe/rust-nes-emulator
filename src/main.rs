mod bus;
mod cartridge;
mod cpu;
mod mapper;
mod ppu;

use bus::Bus;
use std::fs::{read, File};
use std::io::{BufReader, Read};
use std::str;

#[derive(Debug)]
struct Test {
    name: [u8; 5],
    value1: [u8; 2],
    value2: u8,
    value3: u8,
}

fn main() {
    // let mut nes = Bus::new();

    // nes.write(0x0101, 10);

    // println!("intruction: {:?}", cpu::get_instruction_by_id(0));

    // if let Some(cpu) = &mut nes.cpu {
    //     let result = cpu.read(0x0101);
    //     println!("cpu: {}", result);
    //     cpu.reset();
    // }

    // println!("nes: {}", nes.read(0x0101, false));

    let file = File::open("test/file_test.txt").unwrap();
    let mut reader = BufReader::new(file);

    // let result = cartridge::read_struct::<Test, BufReader<File>>(reader);
    // match result {
    //     Ok(data) => {
    //         println!("Name: {:?}", String::from_utf8_lossy(&data.name));
    //         // println!("Value1 LO: {}", (data.value1 as u8) as char);
    //         // println!("Value1 HI: {}", ((data.value1 >> 8) as u8) as char);
    //         println!("Value1: {}", String::from_utf8_lossy(&data.value1));
    //         println!("Value2: {}", data.value2 as char);
    //         println!("Value3: {}", data.value3 as char);
    //     }
    //     Err(e) => println!("Error: {}", e),
    // }

    let mut buffer = vec![];
    buffer.resize((5) as usize, 0);
    reader.read(&mut buffer);
    println!("Name: {:?}", String::from_utf8_lossy(&buffer));
}
