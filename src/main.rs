use std::cell::RefCell;

extern crate nes;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <rom file>", args[0]);
        std::process::exit(1);
    }

    let info = nes::ines::INesInfo::new(&args[1]);
    let cartridge = nes::cartridge::Cartridge::new(info);
    let memory = nes::memory::Memory::new(&cartridge);
    let mut cpu = nes::cpu::CPU::new(RefCell::new(memory));

    for _ in 0..20 {
        cpu.step();
    }
}
