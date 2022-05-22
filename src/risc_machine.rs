use crate::riscv_instruction::{
    ITypeIImmediateInstruction, RTypeInstructionFormat, STypeBImmediateInstruction,
    STypeSImmediateInstruction, UTypeJImmediateInstruction,
};
use ux::{u20, u3, u5, u7};

trait RISCVInstruction {
    fn get_opcode(self) -> u7;
}

impl RISCVInstruction for u32 {
    fn get_opcode(self) -> u7 {
        u7::new((self & 0b1111111) as u8)
    }
}

impl TryFrom<u32> for CPUInstruction {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let opcode = value.get_opcode();
        match u8::from(opcode) {
            0b0110011 => {
                let parsed = RTypeInstructionFormat::from(value);
                if parsed.funct3 == u3::new(0b000) {
                    if parsed.funct7 == u7::new(0b0000000) {
                        return Ok(CPUInstruction::ADD {
                            rd: parsed.rd,
                            rs1: parsed.rs1,
                            rs2: parsed.rs2,
                        });
                    }
                    if parsed.funct7 == u7::new(0b0100000) {
                        return Ok(CPUInstruction::SUB {
                            rd: parsed.rd,
                            rs1: parsed.rs1,
                            rs2: parsed.rs2,
                        });
                    }
                }
                Err(())
            }
            0b0100011 => {
                let parsed = STypeSImmediateInstruction::from(value);
                if parsed.funct3 != u3::new(0b010) {
                    return Err(());
                }
                Ok(CPUInstruction::SW {
                    rs1: parsed.rs1,
                    rs2: parsed.rs2,
                    imm: parsed.imm,
                })
            }
            0b0000011 => {
                let parsed = ITypeIImmediateInstruction::from(value);
                if parsed.funct3 != u3::new(0b010) {
                    return Err(());
                }
                Ok(CPUInstruction::LW {
                    rd: parsed.rd,
                    rs1: parsed.rs1,
                    imm: parsed.imm,
                })
            }
            0b1101111 => {
                let parsed = UTypeJImmediateInstruction::from(value);
                Ok(CPUInstruction::JAL {
                    rd: parsed.rd,
                    imm: parsed.imm,
                })
            }
            0b1100111 => {
                let parsed = ITypeIImmediateInstruction::from(value);
                Ok(CPUInstruction::JALR {
                    rd: parsed.rd,
                    rs1: parsed.rs1,
                    imm: parsed.imm,
                })
            }
            0b1100011 => {
                let parsed = STypeBImmediateInstruction::from(value);
                if parsed.funct3 == u3::new(0b000) {
                    return Ok(CPUInstruction::BEQ {
                        rs1: parsed.rs1,
                        rs2: parsed.rs2,
                        imm: parsed.imm,
                    });
                }
                if parsed.funct3 == u3::new(0b001) {
                    return Ok(CPUInstruction::BNE {
                        rs1: parsed.rs1,
                        rs2: parsed.rs2,
                        imm: parsed.imm,
                    });
                }

                Err(())
            }
            _ => Err(()),
        }
    }
}

impl From<CPUInstruction> for u32 {
    fn from(instruction: CPUInstruction) -> Self {
        match instruction {
            CPUInstruction::ADD { rd, rs1, rs2 } => RTypeInstructionFormat {
                opcode: u7::new(0b0110011),
                rd,
                funct3: u3::new(0b000),
                rs1,
                funct7: u7::new(0b0000000),
                rs2,
            }
            .into(),
            CPUInstruction::SUB { rd, rs1, rs2 } => RTypeInstructionFormat {
                opcode: u7::new(0b0110011),
                rd,
                funct3: u3::new(0b000),
                rs1,
                funct7: u7::new(0b0100000),
                rs2,
            }
            .into(),
            CPUInstruction::LW { rd, rs1, imm } => ITypeIImmediateInstruction {
                opcode: u7::new(0b0000011),
                rd,
                funct3: u3::new(0b010),
                rs1,
                imm,
            }
            .into(),
            CPUInstruction::SW { rs1, rs2, imm } => STypeSImmediateInstruction {
                opcode: u7::new(0b0100011),
                funct3: u3::new(0b010),
                rs1,
                rs2,
                imm,
            }
            .into(),
            CPUInstruction::JAL { rd, imm } => UTypeJImmediateInstruction {
                opcode: u7::new(0b1101111),
                rd,
                imm,
            }
            .into(),
            CPUInstruction::JALR { rd, rs1, imm } => ITypeIImmediateInstruction {
                opcode: u7::new(0b1100111),
                rd,
                funct3: u3::new(0b000),
                rs1,
                imm,
            }
            .into(),
            CPUInstruction::BEQ { rs1, rs2, imm } => STypeBImmediateInstruction {
                opcode: u7::new(0b1100011),
                funct3: u3::new(0b000),
                rs1,
                rs2,
                imm,
            }
            .into(),
            CPUInstruction::BNE { rs1, rs2, imm } => STypeBImmediateInstruction {
                opcode: u7::new(0b1100011),
                funct3: u3::new(0b001),
                rs1,
                rs2,
                imm,
            }
            .into(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CPUInstruction {
    ADD { rd: u5, rs1: u5, rs2: u5 },
    SUB { rd: u5, rs1: u5, rs2: u5 },
    LW { rd: u5, rs1: u5, imm: u32 },
    SW { rs1: u5, rs2: u5, imm: u32 },
    JAL { rd: u5, imm: u32 },
    JALR { rd: u5, rs1: u5, imm: u32 },
    BEQ { rs1: u5, rs2: u5, imm: u32 },
    BNE { rs1: u5, rs2: u5, imm: u32 },
}

#[derive(Default, Debug)]
pub struct Registry([u32; 32]);

impl Registry {
    pub fn set(&mut self, index: u5, value: u32) {
        if u32::from(index) == 0 {
            return;
        }
        self.0[u32::from(index) as usize] = value;
    }

    pub fn get(&self, index: u5) -> u32 {
        self.0[u32::from(index) as usize]
    }
}

#[derive(Debug)]
pub struct Memory(Vec<u8>);

impl Default for Memory {
    fn default() -> Self {
        Memory(vec![0; u32::from(u20::MAX) as usize])
    }
}

impl Memory {
    pub fn set(&mut self, index: u20, value: u8) {
        self.0[u32::from(index) as usize] = value;
    }

    pub fn get(&mut self, index: u20) -> u8 {
        self.0[u32::from(index) as usize]
    }

    pub fn set_four_byte(&mut self, index: u20, value: u32) {
        let start = u32::from(index) as usize;
        self.0[start] = ((value >> 24) & 0xff) as u8;
        self.0[start + 1] = ((value >> 16) & 0xff) as u8;
        self.0[start + 2] = ((value >> 8) & 0xff) as u8;
        self.0[start + 3] = ((value) & 0xff) as u8;
    }

    pub fn get_aligned(&self, index: u20) -> u32 {
        let mut out: u32 = 0;
        let start = u32::from(index) as usize;
        out |= (self.0[start] as u32) << 24;
        out |= (self.0[start + 1] as u32) << 16;
        out |= (self.0[start + 2] as u32) << 8;
        out |= self.0[start + 3] as u32;
        out
    }
}

#[derive(Debug, Default)]
pub struct RISCMachine {
    pub memory: Memory,
    pub registry: Registry,
    pub program_counter: u20,
}

impl RISCMachine {
    pub fn load_memory(&mut self, memory: Memory) {
        self.memory = memory;
    }

    fn execute(&mut self, instruction: CPUInstruction) -> Result<(), ()> {
        match instruction {
            CPUInstruction::ADD { rd, rs1, rs2 } => {
                let value = self
                    .registry
                    .get(rs1)
                    .saturating_add(self.registry.get(rs2));
                self.registry.set(rd, value);
                self.program_counter = self.program_counter.wrapping_add(u20::new(4));
                Ok(())
            }
            CPUInstruction::SUB { rd, rs1, rs2 } => {
                let value = self
                    .registry
                    .get(rs1)
                    .saturating_sub(self.registry.get(rs2));
                self.registry.set(rd, value);
                self.program_counter = self.program_counter.wrapping_add(u20::new(4));
                Ok(())
            }
            CPUInstruction::LW { rd, rs1, imm } => {
                let value = self.memory.get_aligned(u20::new(u32::from(rs1) + imm));
                self.registry.set(rd, value);
                self.program_counter = self.program_counter.wrapping_add(u20::new(4));
                Ok(())
            }
            CPUInstruction::SW { rs1, rs2, imm } => {
                let value = self.registry.get(rs2);
                let index = self.registry.get(rs1) + imm;
                self.memory.set_four_byte(u20::new(index), value);
                self.program_counter = self.program_counter.wrapping_add(u20::new(4));
                Ok(())
            }
            CPUInstruction::JAL { rd, imm } => {
                self.registry.set(
                    rd,
                    u32::from(self.program_counter.wrapping_add(u20::new(4))),
                );

                self.program_counter =
                    u20::new(((u32::from(self.program_counter) as i32) + imm as i32) as u32);
                Ok(())
            }
            CPUInstruction::JALR { rd, rs1, imm } => {
                self.registry.set(
                    rd,
                    u32::from(self.program_counter.wrapping_add(u20::new(4))),
                );

                let value = (self.registry.get(rs1) + imm) & 0xfffffffe;
                if value > u32::from(u20::MAX) {
                    return Err(());
                }
                self.program_counter = u20::new(value);
                Ok(())
            }
            CPUInstruction::BEQ { rs1, rs2, imm } => {
                if self.registry.get(rs1) == self.registry.get(rs2) {
                    self.program_counter =
                        u20::new(((u32::from(self.program_counter) as i32) + imm as i32) as u32);
                } else {
                    self.program_counter = self.program_counter.wrapping_add(u20::new(4));
                }
                Ok(())
            }
            CPUInstruction::BNE { rs1, rs2, imm } => {
                if self.registry.get(rs1) != self.registry.get(rs2) {
                    self.program_counter =
                        u20::new(((u32::from(self.program_counter) as i32) + imm as i32) as u32);
                } else {
                    self.program_counter = self.program_counter.wrapping_add(u20::new(4));
                }
                Ok(())
            }
        }
    }

    fn get_next_instruction(&self) -> u32 {
        self.memory.get_aligned(self.program_counter)
    }

    pub fn tick(&mut self) -> Result<(), ()> {
        let instruction = self.get_next_instruction();
        self.execute(instruction.try_into()?)?;
        Ok(())
    }

    pub fn run(&mut self, until: &dyn Fn(u20) -> bool) -> Result<(), ()> {
        loop {
            self.tick()?;
            if until(self.program_counter) {
                break;
            }
        }

        Ok(())
    }
}
