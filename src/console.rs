struct Console {
    ppu: ::ppu::PPU,
    cart: ::cartridge::Cartridge,
    ram: Vec<u8>
}
