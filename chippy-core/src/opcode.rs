#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Cls,               // 00E0
    Ret,               // 00EE
    Jp(u16),           // 1NNN
    Call(u16),         // 2NNN
    SeVxByte(u8, u8),  // 3XNN  Vx, byte
    SneVxByte(u8, u8), // 4XNN
    SeVxVy(u8, u8),    // 5XY0
    LdVxByte(u8, u8),  // 6XNN
    AddVxByte(u8, u8), // 7XNN
    LdVxVy(u8, u8),    // 8XY0
    OrVxVy(u8, u8),    // 8XY1
    AndVxVy(u8, u8),   // 8XY2
    XorVxVy(u8, u8),   // 8XY3
    AddVxVy(u8, u8),   // 8XY4
    SubVxVy(u8, u8),   // 8XY5
    ShrVx(u8),         // 8XY6
    SubnVxVy(u8, u8),  // 8XY7
    ShlVx(u8),         // 8XYE
    SneVxVy(u8, u8),   // 9XY0
    LdI(u16),          // ANNN
    JpV0(u16),         // BNNN
    Rnd(u8, u8),       // CXNN
    Drw(u8, u8, u8),   // DXYN  Vx, Vy, n
    Skp(u8),           // EX9E
    Sknp(u8),          // EXA1
    LdVxDt(u8),        // FX07
    LdVxK(u8),         // FX0A
    LdDtVx(u8),        // FX15
    LdStVx(u8),        // FX18
    AddIVx(u8),        // FX1E
    LdFVx(u8),         // FX29
    LdBVx(u8),         // FX33
    LdIVx(u8),         // FX55
    LdVxI(u8),         // FX65
    Unknown(u16),      // fallback for unrecognized opcodes
}
impl Instruction {
    pub fn decode(opcode: u16) -> Self {
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F,
        );
        let x = nibbles.1 as u8;
        let y = nibbles.2 as u8;
        let n = nibbles.3 as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => Instruction::Cls,
            (0x0, 0x0, 0xE, 0xE) => Instruction::Ret,
            (0x1, _, _, _) => Instruction::Jp(nnn),
            (0x2, _, _, _) => Instruction::Call(nnn),
            (0x3, _, _, _) => Instruction::SeVxByte(x, nn),
            (0x4, _, _, _) => Instruction::SneVxByte(x, nn),
            (0x5, _, _, 0x0) => Instruction::SeVxVy(x, y),
            (0x6, _, _, _) => Instruction::LdVxByte(x, nn),
            (0x7, _, _, _) => Instruction::AddVxByte(x, nn),
            (0x8, _, _, 0x0) => Instruction::LdVxVy(x, y),
            (0x8, _, _, 0x1) => Instruction::OrVxVy(x, y),
            (0x8, _, _, 0x2) => Instruction::AndVxVy(x, y),
            (0x8, _, _, 0x3) => Instruction::XorVxVy(x, y),
            (0x8, _, _, 0x4) => Instruction::AddVxVy(x, y),
            (0x8, _, _, 0x5) => Instruction::SubVxVy(x, y),
            (0x8, _, _, 0x6) => Instruction::ShrVx(x),
            (0x8, _, _, 0x7) => Instruction::SubnVxVy(x, y),
            (0x8, _, _, 0xE) => Instruction::ShlVx(x),
            (0x9, _, _, 0x0) => Instruction::SneVxVy(x, y),
            (0xA, _, _, _) => Instruction::LdI(nnn),
            (0xB, _, _, _) => Instruction::JpV0(nnn),
            (0xC, _, _, _) => Instruction::Rnd(x, nn),
            (0xD, _, _, _) => Instruction::Drw(x, y, n),
            (0xE, _, 0x9, 0xE) => Instruction::Skp(x),
            (0xE, _, 0xA, 0x1) => Instruction::Sknp(x),
            (0xF, _, 0x0, 0x7) => Instruction::LdVxDt(x),
            (0xF, _, 0x0, 0xA) => Instruction::LdVxK(x),
            (0xF, _, 0x1, 0x5) => Instruction::LdDtVx(x),
            (0xF, _, 0x1, 0x8) => Instruction::LdStVx(x),
            (0xF, _, 0x1, 0xE) => Instruction::AddIVx(x),
            (0xF, _, 0x2, 0x9) => Instruction::LdFVx(x),
            (0xF, _, 0x3, 0x3) => Instruction::LdBVx(x),
            (0xF, _, 0x5, 0x5) => Instruction::LdIVx(x),
            (0xF, _, 0x6, 0x5) => Instruction::LdVxI(x),
            _ => Instruction::Unknown(opcode),
        }
    }
}
