use crate::display::Display;
use crate::keypad::Keypad;
use crate::memory::Memory;
use crate::opcode::Instruction;

const STACK_SIZE: usize = 16;
pub struct Cpu {
    pub registers: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub stack: [u16; STACK_SIZE],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn reset(&mut self) {
        self.registers = [0; 16];
        self.i = 0;
        self.pc = 0x200;
        self.stack = [0; STACK_SIZE];
        self.sp = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
    }

    fn push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn tick(&mut self, memory: &mut Memory, display: &mut Display, keypad: &Keypad) {
        let opcode = self.fetch(memory);
        let current_instruction = Instruction::decode(opcode);
        self.execute(current_instruction, memory, display, keypad);
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
    fn fetch(&mut self, memory: &Memory) -> u16 {
        let opcode: u16 = memory.read_word(self.pc);
        self.pc = self.pc.wrapping_add(2);
        opcode
    }
    pub fn is_sound_playing(&self) -> bool {
        self.sound_timer > 0
    }

    fn execute(
        &mut self,
        instruction: Instruction,
        memory: &mut Memory,
        display: &mut Display,
        keypad: &Keypad,
    ) {
        match instruction {
            Instruction::Cls => {
                display.clear();
            }
            Instruction::Ret => self.op_ret(),
            Instruction::Jp(addr) => self.pc = addr,
            Instruction::Call(addr) => self.op_call(addr),
            Instruction::SeVxByte(x, kk) => {
                if self.registers[x as usize] == kk {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            Instruction::SneVxByte(x, kk) => {
                if self.registers[x as usize] != kk {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            Instruction::SeVxVy(x, y) => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            Instruction::LdVxByte(x, kk) => self.registers[x as usize] = kk,
            Instruction::AddVxByte(x, kk) => {
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(kk)
            }
            Instruction::LdVxVy(x, y) => self.registers[x as usize] = self.registers[y as usize],
            Instruction::OrVxVy(x, y) => self.registers[x as usize] |= self.registers[y as usize],
            Instruction::AndVxVy(x, y) => self.registers[x as usize] &= self.registers[y as usize],
            Instruction::XorVxVy(x, y) => self.registers[x as usize] ^= self.registers[y as usize],
            Instruction::AddVxVy(x, y) => self.op_add_vx_vy(x, y),
            Instruction::SubVxVy(x, y) => self.op_sub_vx_vy(x, y),
            Instruction::ShrVx(x) => self.op_shr_vx(x),
            Instruction::SubnVxVy(x, y) => self.op_subn_vx_vy(x, y),
            Instruction::ShlVx(x) => self.op_shl_vx(x),
            Instruction::SneVxVy(x, y) => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            Instruction::LdI(nnn) => self.i = nnn,
            Instruction::JpV0(nnn) => self.pc = nnn + self.registers[0] as u16,
            Instruction::Rnd(x, kk) => self.op_rnd(x, kk),
            Instruction::Drw(x, y, n) => self.op_dxyn(x, y, n, memory, display),
            Instruction::Skp(x) => {
                if keypad.is_key_pressed(self.registers[x as usize]) {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            Instruction::Sknp(x) => {
                if !keypad.is_key_pressed(self.registers[x as usize]) {
                    self.pc = self.pc.wrapping_add(2);
                }
            }
            Instruction::LdVxDt(x) => self.registers[x as usize] = self.delay_timer,
            Instruction::LdVxK(x) => self.op_ld_vx_k(x, keypad),
            Instruction::LdDtVx(x) => self.delay_timer = self.registers[x as usize],
            Instruction::LdStVx(x) => self.sound_timer = self.registers[x as usize],
            Instruction::AddIVx(x) => {
                self.i = self.i.wrapping_add(self.registers[x as usize] as u16)
            }
            Instruction::LdFVx(x) => self.i = memory.font_address(self.registers[x as usize]),
            Instruction::LdBVx(x) => self.op_ld_b_vx(x, memory),
            Instruction::LdIVx(x) => self.op_ld_i_vx(x, memory),
            Instruction::LdVxI(x) => self.op_ld_vx_i(x, memory),
            Instruction::Unknown(op) => {
                eprintln!("Unknown opcode: {:#06X} at PC: {:#06X}", op, self.pc - 2)
            }
        }
    }

    fn op_call(&mut self, addr: u16) {
        self.push(self.pc);
        self.pc = addr;
    }
    fn op_ret(&mut self) {
        self.pc = self.pop();
    }
    fn op_add_vx_vy(&mut self, x: u8, y: u8) {
        let (result, carry) =
            self.registers[x as usize].overflowing_add(self.registers[y as usize]);
        self.registers[0xF] = carry as u8;
        self.registers[x as usize] = result;
    }
    fn op_sub_vx_vy(&mut self, x: u8, y: u8) {
        let (result, borrow) =
            self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
        self.registers[x as usize] = result;
        self.registers[0xF] = !borrow as u8
    }
    fn op_shr_vx(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        self.registers[0xF] = vx & 0x1;
        self.registers[x as usize] = vx >> 1;
    }
    fn op_subn_vx_vy(&mut self, x: u8, y: u8) {
        let (result, borrow) =
            self.registers[y as usize].overflowing_sub(self.registers[x as usize]);
        self.registers[x as usize] = result;
        self.registers[0xF] = !borrow as u8;
    }
    fn op_shl_vx(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        self.registers[0xF] = (vx & 0x80) >> 7;
        self.registers[x as usize] = vx << 1;
    }
    fn op_rnd(&mut self, x: u8, kk: u8) {
        let random_byte: u8 = rand::random();
        self.registers[x as usize] = random_byte & kk;
    }
    fn op_dxyn(&mut self, x: u8, y: u8, n: u8, memory: &Memory, display: &mut Display) {
        let vx = self.registers[x as usize] as usize;
        let vy = self.registers[y as usize] as usize;
        let sprite = memory.read_slice(self.i, n);
        let collision = display.draw_sprite(vx, vy, sprite);
        self.registers[0xF] = collision as u8;
    }
    fn op_ld_vx_k(&mut self, x: u8, keypad: &Keypad) {
        if let Some(key) = keypad.first_pressed_key() {
            self.registers[x as usize] = key;
        } else {
            self.pc = self.pc.wrapping_sub(2);
        }
    }
    fn op_ld_b_vx(&mut self, x: u8, memory: &mut Memory) {
        let value = self.registers[x as usize];
        memory.write_bcd(self.i, value);
    }
    fn op_ld_i_vx(&mut self, x: u8, memory: &mut Memory) {
        let count = x as usize + 1;
        memory.write_registers(self.i, &self.registers[0..count]);
    }
    fn op_ld_vx_i(&mut self, x: u8, memory: &Memory) {
        let count = x as usize + 1;
        let bytes = memory.read_registers(self.i, count);
        self.registers[0..count].copy_from_slice(bytes);
    }
}
