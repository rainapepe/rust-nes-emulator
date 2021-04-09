mod bus;
mod cpu;
mod ppu;

use bus::Bus;

fn main() {
    let mut nes = Bus::new();

    nes.write(0x0101, 10);

    println!("intruction: {:?}", cpu::get_instruction_by_id(0));

    if let Some(cpu) = &mut nes.cpu {
        let result = cpu.read(0x0101);
        println!("cpu: {}", result);
        cpu.reset();
    }

    println!("nes: {}", nes.read(0x0101, false));
}
