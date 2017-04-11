extern crate nes;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if (args.len() != 2) {
        println!("usage: {} <rom file>", args[0]);
        std::process::exit(1);
    }


    let cartridge = nes::cartridge::Cartridge::new(&args[1]);
    let memory = nes::memory::Memory::new(&cartridge);
    let cpu = nes::cpu::CPU::new(&memory);
    println!("Hello, World!");
}
