#![allow(dead_code)]
pub const MEMORY_SIZE: usize = 4096;
pub const PROGRAM_START: u16 = 0x200; // most ROMs load here
pub const FONT_START: u16 = 0x050; // common convention (0x000-0x1FF reserved for interpreter)
pub const FONT_SIZE: usize = 80; // 16 chars * 5 bytes each

const FONT_SET: [u8; FONT_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
    ram: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Memory {
            ram: [0; MEMORY_SIZE],
        };
        memory.load_font();
        return memory;
    }

    fn load_font(&mut self) {
        for (i, &byte) in FONT_SET.iter().enumerate() {
            self.ram[FONT_START as usize + i] = byte;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        let start = PROGRAM_START as usize;
        let end = start + rom.len();
        assert!(end <= MEMORY_SIZE, "ROM too large to fit in memory");
        self.ram[start..end].copy_from_slice(rom);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    /// Two-byte read — used by Cpu::fetch to grab an opcode (big-endian)
    pub fn read_word(&self, addr: u16) -> u16 {
        let hi = self.ram[addr as usize] as u16;
        let lo = self.ram[addr as usize + 1] as u16;
        (hi << 8) | lo
    }

    /// Returns the memory address of the font sprite for a given hex digit (0-F)
    /// Used by the FX29 opcode ("LD F, Vx")
    pub fn font_address(&self, digit: u8) -> u16 {
        FONT_START + (digit as u16) * 5
    }

    /// Read a slice of bytes — handy for DXYN sprite drawing (reads N bytes from I)
    pub fn read_slice(&self, addr: u16, len: u8) -> &[u8] {
        let start = addr as usize;
        let end = start + len as usize;
        &self.ram[start..end]
    }

    /// BCD storage for FX33 — writes hundreds/tens/ones digits of Vx to I, I+1, I+2
    pub fn write_bcd(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value / 100;
        self.ram[addr as usize + 1] = (value / 10) % 10;
        self.ram[addr as usize + 2] = value % 10;
    }

    /// FX55 — store registers V0..=Vx into memory starting at I
    pub fn write_registers(&mut self, addr: u16, registers: &[u8]) {
        let start = addr as usize;
        self.ram[start..start + registers.len()].copy_from_slice(registers);
    }

    /// FX65 — load memory starting at I into registers V0..=Vx
    pub fn read_registers(&self, addr: u16, count: usize) -> &[u8] {
        let start = addr as usize;
        &self.ram[start..start + count]
    }
}
