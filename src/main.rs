use std::cell::RefCell;
use std::rc::Rc;

use nes::ines::INesInfo;
use nes::cartridge::Cartridge;
use nes::ppu::PPU;
use nes::memory::CPUMemoryMap;
use nes::cpu::{CPU,Interrupt};


extern crate nes;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <rom file>", args[0]);
        std::process::exit(1);
    }

    let info = INesInfo::new(&args[1]);
    let cartridge = Cartridge::new(info);
    let cartridge = Rc::new(RefCell::new(cartridge));
    let ppu = PPU::new(cartridge.clone());
    let memory_map = CPUMemoryMap::new(cartridge, ppu);
    let mut cpu = CPU::new(memory_map);

    for _ in 0..40000 {
        let int = if (cpu.memory.ppu.nmi == true) { cpu.memory.ppu.nmi = false; Interrupt::IntNMI } else { Interrupt::IntNone };
        let cycles = cpu.step(int);
        cpu.memory.ppu.step(cycles*3);
    }
}
