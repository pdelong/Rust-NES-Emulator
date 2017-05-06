extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;

use nes::ines::INesInfo;
use nes::cartridge::Cartridge;
use nes::ppu::PPU;
use nes::memory::CPUMemoryMap;
use nes::cpu::{CPU,Interrupt};

use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate nes;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <rom file>", args[0]);
        std::process::exit(1);
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("NES Emulator", 512, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::BGR24, 256, 240).unwrap();

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..240 {
            for x in 0..256 {
                let offset = y*pitch + x*3;
                buffer[offset + 0] = 0 as u8;
                buffer[offset + 1] = 0 as u8;
                buffer[offset + 2] = 0;
            }
        }
    }).unwrap();

    renderer.clear();
    renderer.copy(&texture, None, None).unwrap();
    renderer.present();

    let info = INesInfo::new(&args[1]);
    let cartridge = Cartridge::new(info);
    let cartridge = Rc::new(RefCell::new(cartridge));
    let ppu = PPU::new(cartridge.clone());
    let memory_map = CPUMemoryMap::new(cartridge, ppu);
    let mut cpu = CPU::new(memory_map);

    for _ in 0..100000 {
        if cpu.memory.ppu.nmi {
            texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..240 {
                    for x in 0..256 {
                        let offset = y*pitch + x*3;
                        buffer[offset + 0] = cpu.memory.ppu.pixeldata[offset + 0];
                        buffer[offset + 1] = cpu.memory.ppu.pixeldata[offset + 1];
                        buffer[offset + 2] = cpu.memory.ppu.pixeldata[offset + 2];
                    }
                }
            }).unwrap();

            renderer.clear();
            renderer.copy(&texture, None, None).unwrap();
            renderer.present();
        }
        let int = if (cpu.memory.ppu.nmi == true) { cpu.memory.ppu.nmi = false; Interrupt::IntNMI } else { Interrupt::IntNone };
        let cycles = cpu.step(int);
        cpu.memory.ppu.step(cycles*3);
    }
}
