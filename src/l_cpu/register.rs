// registers

#[derive(Debug, Default, Clone)]
pub struct Registers{
    pub gpr: [u32; 13], // R0-R12
    pub sp: u32,        // R13
    pub lr: u32,        // R14
    pub pc: u32,        // R15
    pub psr: PSR,       // Status Register
}

#[derive(Debug, Default, Clone)]
pub struct PSR {
    pub z: bool,
    pub n: bool,
    pub c: bool,
    pub v: bool,
    pub i: bool, // IRQ mask
    pub f: bool, // FIQ mask
    pub p: bool, // Privilege bit
}

impl PSR {
    pub fn to_u32(&self) -> u32 {
        let mut v: u32 = 0;
        if self.n { v |= 1 << 31; }
        if self.z { v |= 1 << 30; }
        if self.c { v |= 1 << 29; }
        if self.v { v |= 1 << 28; }
        if self.i { v |= 1 << 7; }
        if self.f { v |= 1 << 6; }
        if self.p { v |= 1 << 0; }
        v
    }

    pub fn from_u32(raw: u32) -> Self {
        Self {
            n: (raw >> 31) & 1 == 1,
            z: (raw >> 30) & 1 == 1,
            c: (raw >> 29) & 1 == 1,
            v: (raw >> 28) & 1 == 1,
            i: (raw >> 7) & 1 == 1,
            f: (raw >> 6) & 1 == 1,
            p: (raw & 1) == 1,
        }
    }

    pub fn set_zncv_from_result(&mut self, result: u32, carry: bool, overflow: bool) {
        self.z = result == 0;
        self.n = (result >> 31) & 1 == 1;
        self.c = carry;
        self.v = overflow;
    }
}

impl Registers {
    pub fn get(&self, reg: u8) -> u32 {
        match reg {
            0..=12 => self.gpr[reg as usize],
            13 => self.sp,
            14 => self.lr,
            15 => self.pc,
            _ => 0,
        }
    }

    pub fn set(&mut self, reg: u8, value: u32) {
        match reg {
            0..=12 => self.gpr[reg as usize] = value,
            13 => self.sp = value,
            14 => self.lr = value,
            15 => self.pc = value & !0x3, // PC-aligned to 4 bytes
            _ => {}
        }
    }
}