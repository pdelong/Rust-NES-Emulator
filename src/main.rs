use std::cell::RefCell;
use std::rc::Rc;

extern crate nes;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <rom file>", args[0]);
        std::process::exit(1);
    }

    let info = nes::ines::INesInfo::new(&args[1]);
    let cartridge = nes::cartridge::Cartridge::new(info);
    let cartridge = Rc::new(RefCell::new(cartridge));
    let ppu = nes::ppu::PPU::new(cartridge.clone());
    let memory_map = nes::memory::CPUMemoryMap::new(cartridge, ppu);
    let mut cpu = nes::cpu::CPU::new(memory_map);

    for _ in 0..20000 {
        cpu.step();
    }
}
