use super::register::{Registers, PSR};
use super::isa::{DecodedInstr, Condition, Class};

#[derive(Debug, Default, Clone)]
pub struct SystemRegs {
    pub vtbr: u32,     // Vector Table Base Register (1 KiB aligned)
    pub scr: u32,      // System Control Register (caches/MMU/mode bits)
    pub ttbr0: u32,
    pub ttbr1: u32,
    pub asid: u32,
    pub far: u32,
    pub ifsr: u32,
    pub dfsr: u32,
    pub cnt: u64,      // cycle/wall counter
    pub cnt_cmp: u64,  // compare
    pub epc: u32,      // exception PC (banked)
    pub epsr: u32,     // exception PSR (banked)
}

pub struct CPU {
    pub regs: Registers,
    pub sys: SystemRegs,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = Self { regs: Registers::default(), sys: SystemRegs::default() };
        cpu.reset();
        cpu
    }
    
    pub fn reset(&mut self) {
        // Reset state per spec
        self.regs = Registers::default();
        self.sys = SystemRegs::default();
        // Reset sets PL1 (kernel), caches/MMU disabled, PSR = 0x0000_0000
        let mut psr = PSR::default();
        psr.p = true; // PL1
        self.regs.psr = psr;
        self.sys.vtbr = 0;
        self.regs.pc = self.sys.vtbr + 0x00; // Reset vector
    }

    pub fn run(&mut self) {
        println!("CPU: Running Instruction Cycle...");
        // For now, this is a stub. Integrate with memory/bus to fetch instructions.
    }

    pub fn step_with_instr(&mut self, instr_word: u32) {
        // Execute a single instruction provided by caller (useful until bus is wired)
        let instr = DecodedInstr::decode(instr_word);
        if !self.condition_passed(instr.cond) {
            self.advance_pc();
            return;
        }
        match instr.class {
            Class::Alu => self.exec_alu(instr),
            Class::MulDiv => self.exec_muldiv(instr),
            Class::LoadStore => self.exec_loadstore(instr),
            Class::Branch => self.exec_branch(instr),
            Class::System => self.exec_system(instr),
        }
    }

    fn advance_pc(&mut self) { self.regs.pc = self.regs.pc.wrapping_add(4); }

    fn condition_passed(&self, cond: Condition) -> bool {
        use Condition::*;
        let psr = &self.regs.psr;
        match cond {
            AL => true,
            NV => false,
            EQ => psr.z,
            NE => !psr.z,
            CS => psr.c,
            CC => !psr.c,
            MI => psr.n,
            PL => !psr.n,
            VS => psr.v,
            VC => !psr.v,
            HI => psr.c && !psr.z,
            LS => !psr.c || psr.z,
            GE => psr.n == psr.v,
            LT => psr.n != psr.v,
            GT => !psr.z && (psr.n == psr.v),
            LE => psr.z || (psr.n != psr.v),
        }
    }

    fn exec_alu(&mut self, di: DecodedInstr) {
        // Minimal subset: MOV/MVN, ADD/SUB, AND/ORR/EOR, CMP/TST, shifts (imm, simple)
        let opcode = di.opcode;
        let rn = self.regs.get(di.rn);
        let rm = self.regs.get(di.rm);
        let imm = di.imm12 as u32;
        let (result, carry, overflow, write_rd, is_cmp_like) = match opcode {
            0x0 => (rn.wrapping_add(imm), (rn as u64 + imm as u64) > 0xFFFF_FFFF, (((rn ^ !imm) & (rn ^ (rn.wrapping_add(imm)))) >> 31) & 1 == 1, true, false), // ADD (imm)
            0x1 => (rn.wrapping_sub(imm), (rn as u64) < (imm as u64), (((rn ^ imm) & (rn ^ (rn.wrapping_sub(imm)))) >> 31) & 1 == 1, true, false), // SUB (imm)
            0x2 => (rn & rm, false, false, true, false), // AND (reg)
            0x3 => (rn | rm, false, false, true, false), // ORR (reg)
            0x4 => (rn ^ rm, false, false, true, false), // EOR (reg)
            0x5 => (imm, false, false, true, false),     // MOV (imm)
            0x6 => (!rm, false, false, true, false),     // MVN (reg)
            0x7 => (rn, false, false, false, true),       // TST (rn & rm to flags)
            0x8 => (rn, false, false, false, true),       // CMP (rn - rm to flags)
            _ => { self.advance_pc(); return; }
        };

        if is_cmp_like {
            if opcode == 0x7 { // TST
                let v = rn & rm;
                self.regs.psr.set_zncv_from_result(v, self.regs.psr.c, self.regs.psr.v);
            } else { // CMP
                let sub = rn.wrapping_sub(rm);
                let borrow = (rn as u64) < (rm as u64);
                let ov = (((rn ^ rm) & (rn ^ sub)) >> 31) & 1 == 1;
                self.regs.psr.set_zncv_from_result(sub, !borrow, ov);
            }
        } else if write_rd {
            self.regs.set(di.rd, result);
            if di.set_flags {
                self.regs.psr.set_zncv_from_result(result, carry, overflow);
            }
        }
        self.advance_pc();
    }

    fn exec_muldiv(&mut self, _di: DecodedInstr) { self.advance_pc(); }

    fn exec_loadstore(&mut self, _di: DecodedInstr) {
        // Placeholder: bus/memory integration required
        self.advance_pc();
    }

    fn exec_branch(&mut self, di: DecodedInstr) {
        // Minimal: B/BL with 24-bit signed immediate from imm12+rd/rn mocked here
        // For now, treat imm12 as a signed 12-bit offset in words
        let imm_sext = ((di.imm12 as i16) as i32 as i32) << 2;
        let link = di.set_flags; // reuse S bit as link for this sketch
        let target = self.regs.pc.wrapping_add(4).wrapping_add(imm_sext as u32);
        if link { self.regs.lr = self.regs.pc.wrapping_add(4); }
        self.regs.pc = target & !0x3;
    }

    fn exec_system(&mut self, di: DecodedInstr) {
        // Minimal: MRS/MSR by opcode, SVC, RFE, WFI no-op
        match di.opcode {
            0x0 => { // MRS rd, PSR
                let v = self.regs.psr.to_u32();
                self.regs.set(di.rd, v);
                self.advance_pc();
            }
            0x1 => { // MSR PSR, rn
                let v = self.regs.get(di.rn);
                self.regs.psr = PSR::from_u32(v);
                self.advance_pc();
            }
            0x2 => { // SVC imm
                self.raise_exception(0x08);
            }
            0x3 => { // RFE
                let psr = PSR::from_u32(self.sys.epsr);
                self.regs.psr = psr;
                self.regs.pc = self.sys.epc & !0x3;
            }
            _ => { self.advance_pc(); }
        }
    }

    fn raise_exception(&mut self, vector_offset: u32) {
        // On entry: save PC and PSR to EPC/EPSR; switch to PL1
        self.sys.epc = self.regs.pc;
        self.sys.epsr = self.regs.psr.to_u32();
        self.regs.psr.p = true;
        let handler = (self.sys.vtbr & !0x3FF) + vector_offset;
        self.regs.pc = handler & !0x3;
    }
}