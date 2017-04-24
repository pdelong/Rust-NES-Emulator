struct Console<'a> {
    cpu: ::cpu::CPU<'a>,
    ppu: ::ppu::PPU<'a>,
    cart: ::cartridge::Cartridge,
    ram: Vec<u8>
}
