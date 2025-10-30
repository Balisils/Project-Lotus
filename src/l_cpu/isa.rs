// ISA definitions and decoding skeleton for L-CPU v1.0

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Condition {
    EQ, // Z == 1
    NE, // Z == 0
    CS, // C == 1
    CC, // C == 0
    MI, // N == 1
    PL, // N == 0
    VS, // V == 1
    VC, // V == 0
    HI, // C == 1 && Z == 0
    LS, // C == 0 || Z == 1
    GE, // N == V
    LT, // N != V
    GT, // Z == 0 && N == V
    LE, // Z == 1 || N != V
    AL, // always
    NV, // reserved/never
}

impl Condition {
    pub fn from_u8(bits: u8) -> Self {
        use Condition::*;
        match bits & 0xF {
            0x0 => EQ,
            0x1 => NE,
            0x2 => CS,
            0x3 => CC,
            0x4 => MI,
            0x5 => PL,
            0x6 => VS,
            0x7 => VC,
            0x8 => HI,
            0x9 => LS,
            0xA => GE,
            0xB => LT,
            0xC => GT,
            0xD => LE,
            0xE => AL,
            _ => NV,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Class {
    Alu = 0,
    MulDiv = 1,
    LoadStore = 2,
    Branch = 3,
    System = 4,
}

#[derive(Debug, Copy, Clone)]
pub struct DecodedInstr {
    pub raw: u32,
    pub cond: Condition,
    pub class: Class,
    pub opcode: u8,
    pub rd: u8,
    pub rn: u8,
    pub rm: u8,
    pub imm12: u16,
    pub set_flags: bool,
}

impl DecodedInstr {
    pub fn decode(raw: u32) -> Self {
        let cond = Condition::from_u8(((raw >> 28) & 0xF) as u8);
        let class_bits = ((raw >> 25) & 0x7) as u8;
        let class = match class_bits {
            0 => Class::Alu,
            1 => Class::MulDiv,
            2 => Class::LoadStore,
            3 => Class::Branch,
            _ => Class::System,
        };
        let opcode = ((raw >> 21) & 0xF) as u8;
        let rd = ((raw >> 12) & 0xF) as u8;
        let rn = ((raw >> 16) & 0xF) as u8;
        let rm = (raw & 0xF) as u8;
        let imm12 = (raw & 0xFFF) as u16;
        let set_flags = ((raw >> 20) & 0x1) != 0;
        Self { raw, cond, class, opcode, rd, rn, rm, imm12, set_flags }
    }
}


