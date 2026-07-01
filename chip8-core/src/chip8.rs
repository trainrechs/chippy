use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use crate::memory::Memory;

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keypad: Keypad,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.load_rom(rom);
    }

    pub fn tick(&mut self) {
        self.cpu
            .tick(&mut self.memory, &mut self.display, &self.keypad);
    }

    pub fn tick_timers(&mut self) {
        self.cpu.tick_timers();
    }

    pub fn is_sound_playing(&self) -> bool {
        self.cpu.is_sound_playing()
    }

    pub fn get_display_buffer(&self) -> &[bool] {
        self.display.get_buffer()
    }

    pub fn is_display_dirty(&self) -> bool {
        self.display.is_dirty()
    }

    pub fn clear_display_dirty(&mut self) {
        self.display.clear_dirty();
    }

    pub fn key_down(&mut self, key: u8) {
        self.keypad.set_pressed(key, true);
    }

    pub fn key_up(&mut self, key: u8) {
        self.keypad.set_pressed(key, false);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
