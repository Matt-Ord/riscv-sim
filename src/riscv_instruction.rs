use ux::{u1, u3, u4, u5, u6, u7};

pub enum RISCVImmediate {
    I,
    S,
    B,
    U,
    J,
}

pub trait RISCVInstruction
where
    Self: Sized,
{
    fn opcode(&self) -> u7;
    fn with_opcode(&self, opcode: u7) -> Self;
    fn rd(&self) -> u5;
    fn with_rd(&self, rd: u5) -> Self;
    fn funct3(&self) -> u3;
    fn with_funct3(&self, funct3: u3) -> Self;
    fn rs1(&self) -> u5;
    fn with_rs1(&self, rs1: u5) -> Self;
    fn rs2(&self) -> u5;
    fn with_rs2(&self, rs2: u5) -> Self;
    fn funct7(&self) -> u7;
    fn with_funct7(&self, funct7: u7) -> Self;

    fn im_7(&self) -> u1 {
        u1::new(u8::from(self.rd()) & 1)
    }
    fn with_im_7(&self, im_7: u1) -> Self {
        let mut rd: u8 = self.rd().into();
        rd &= !(1);
        rd |= u8::from(bool::from(im_7));
        self.with_rd(u5::new(rd))
    }
    fn im_8_11(&self) -> u4 {
        u4::new(u8::from(self.rd()) >> 1)
    }
    fn with_im_8_11(&self, im_8_11: u4) -> Self {
        let mut rd = u8::from(bool::from(self.im_7()));
        rd |= u8::from(im_8_11) << 1;
        self.with_rd(u5::new(rd))
    }
    fn im_12_19(&self) -> u8 {
        (u8::from(self.rs1()) << 3) | u8::from(self.funct3())
    }
    fn with_im_12_19(&self, im_12_19: u8) -> Self {
        self.with_funct3(u3::new(im_12_19 & 0b111))
            .with_rs1(u5::new(im_12_19 >> 3))
    }
    fn im_20(&self) -> u1 {
        u1::new(u8::from(self.rs2()) & 1)
    }
    fn with_im_20(&self, im_20: u1) -> Self {
        let mut rs2: u8 = self.rs2().into();
        rs2 &= !(1);
        rs2 |= u8::from(bool::from(im_20));
        self.with_rs2(u5::new(rs2))
    }
    fn im_21_24(&self) -> u4 {
        u4::new(u8::from(self.rs2()) >> 1)
    }
    fn with_im_21_24(&self, im_21_24: u4) -> Self {
        let mut rs2 = u8::from(bool::from(self.im_20()));
        rs2 |= u8::from(im_21_24) << 1;
        self.with_rs2(u5::new(rs2))
    }
    fn im_25_30(&self) -> u6 {
        u6::new(u8::from(self.funct7()) & 0b111111)
    }
    fn with_im_25_30(&self, im_25_30: u6) -> Self {
        let mut funct7 = u8::from(bool::from(self.im_31())) << 6;
        funct7 |= u8::from(im_25_30);
        self.with_funct7(u7::new(funct7))
    }
    fn im_31(&self) -> u1 {
        u1::new(u8::from(self.funct7()) >> 6)
    }
    fn with_im_31(&self, im_31: u1) -> Self {
        let mut funct7: u8 = self.funct7().into();
        funct7 &= 0b0111111;
        funct7 |= u8::from(bool::from(im_31)) << 6;
        self.with_funct7(u7::new(funct7))
    }

    fn immediate(&self, encoding: &RISCVImmediate) -> u32 {
        match encoding {
            RISCVImmediate::I => {
                let mut ret: u32 = 0;
                ret |= u32::from(bool::from(self.im_20()));
                ret |= (u32::from(self.im_21_24())) << 1;
                ret |= (u32::from(self.im_25_30())) << 5;
                ret |= if self.im_31().into() {
                    !(0b11111111111)
                } else {
                    0
                };
                ret
            }
            RISCVImmediate::S => {
                let mut ret: u32 = 0;
                ret |= u32::from(bool::from(self.im_7()));
                ret |= (u32::from(self.im_8_11())) << 1;
                ret |= (u32::from(self.im_25_30())) << 5;
                ret |= if self.im_31().into() {
                    !(0b11111111111)
                } else {
                    0
                };
                ret
            }
            RISCVImmediate::B => {
                let mut ret: u32 = 0;
                // lowest bit always zero
                ret |= (u32::from(self.im_8_11())) << 1;
                ret |= (u32::from(self.im_25_30())) << 5;
                ret |= (u32::from(bool::from(self.im_7()))) << 11;
                ret |= if self.im_31().into() {
                    !(0b111111111111)
                } else {
                    0
                };
                ret
            }
            RISCVImmediate::U => {
                let mut ret: u32 = 0;
                // lowest 12 bit always zero
                ret |= (u32::from(self.im_12_19())) << 12;
                ret |= (u32::from(bool::from(self.im_20()))) << 20;
                ret |= (u32::from(self.im_21_24())) << 21;
                ret |= (u32::from(self.im_25_30())) << 25;
                ret |= (u32::from(bool::from(self.im_20()))) << 31;
                ret
            }
            RISCVImmediate::J => {
                let mut ret: u32 = 0;
                // lowest bit always zero
                ret |= (u32::from(self.im_21_24())) << 1;
                ret |= (u32::from(self.im_25_30())) << 5;
                ret |= (u32::from(bool::from(self.im_20()))) << 11;
                ret |= (u32::from(self.im_12_19())) << 12;
                ret |= if self.im_31().into() {
                    !(0b11111111111111111111)
                } else {
                    0
                };
                ret
            }
        }
    }

    fn with_immediate(&self, immediate: u32, encoding: &RISCVImmediate) -> Self {
        match encoding {
            RISCVImmediate::I => self
                .with_im_20(u1::new((immediate & 0b1) as u8))
                .with_im_21_24(u4::new((immediate >> 1 & 0b1111) as u8))
                .with_im_25_30(u6::new((immediate >> 5 & 0b111111) as u8))
                .with_im_31(u1::new((immediate >> 11 & 0b1) as u8)),
            RISCVImmediate::S => self
                .with_im_7(u1::new((immediate & 0b1) as u8))
                .with_im_8_11(u4::new((immediate >> 1 & 0b1111) as u8))
                .with_im_25_30(u6::new((immediate >> 5 & 0b111111) as u8))
                .with_im_31(u1::new((immediate >> 11 & 0b1) as u8)),
            RISCVImmediate::B => self
                .with_im_8_11(u4::new((immediate >> 1 & 0b1111) as u8))
                .with_im_25_30(u6::new((immediate >> 5 & 0b111111) as u8))
                .with_im_7(u1::new((immediate >> 11 & 0b1) as u8))
                .with_im_31(u1::new((immediate >> 12 & 0b1) as u8)),
            RISCVImmediate::U => self
                .with_im_12_19((immediate >> 12 & 0b11111111) as u8)
                .with_im_20(u1::new((immediate >> 20 & 0b1) as u8))
                .with_im_21_24(u4::new((immediate >> 21 & 0b1111) as u8))
                .with_im_25_30(u6::new((immediate >> 25 & 0b111111) as u8))
                .with_im_31(u1::new((immediate >> 31 & 0b1) as u8)),
            RISCVImmediate::J => self
                .with_im_21_24(u4::new((immediate >> 1 & 0b1111) as u8))
                .with_im_25_30(u6::new((immediate >> 5 & 0b111111) as u8))
                .with_im_20(u1::new((immediate >> 11 & 0b1) as u8))
                .with_im_12_19((immediate >> 12 & 0b11111111) as u8)
                .with_im_31(u1::new((immediate >> 20 & 0b1) as u8)),
        }
    }
}

impl RISCVInstruction for u32 {
    fn opcode(&self) -> u7 {
        u7::new((self & 0b1111111) as u8)
    }

    fn with_opcode(&self, opcode: u7) -> u32 {
        let mut ret = *self;
        ret &= 0b0000000;
        ret |= u32::from(opcode);
        ret
    }

    fn rd(&self) -> u5 {
        u5::new(((self >> 7) & 0b11111) as u8)
    }
    fn with_rd(&self, rd: u5) -> u32 {
        let mut ret = *self;
        ret &= !(0b11111 << 7);
        ret |= u32::from(rd) << 7;
        ret
    }
    fn funct3(&self) -> u3 {
        u3::new(((self >> 12) & 0b111) as u8)
    }
    fn with_funct3(&self, funct3: u3) -> u32 {
        let mut ret = *self;
        ret &= !(0b111 << 12);
        ret |= u32::from(funct3) << 12;
        ret
    }
    fn rs1(&self) -> u5 {
        u5::new(((self >> 15) & 0b11111) as u8)
    }
    fn with_rs1(&self, rs1: u5) -> u32 {
        let mut ret = *self;
        ret &= !(0b11111 << 15);
        ret |= u32::from(rs1) << 15;
        ret
    }
    fn rs2(&self) -> u5 {
        u5::new(((self >> 20) & 0b11111) as u8)
    }
    fn with_rs2(&self, rs2: u5) -> u32 {
        let mut ret = *self;
        ret &= !(0b11111 << 20);
        ret |= u32::from(rs2) << 20;
        ret
    }
    fn funct7(&self) -> u7 {
        u7::new(((self >> 25) & 0b1111111) as u8)
    }
    fn with_funct7(&self, funct7: u7) -> u32 {
        let mut ret = *self;
        ret &= !(0b1111111 << 25);
        ret |= u32::from(funct7) << 25;
        ret
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct RTypeInstructionFormat {
    pub opcode: u7,
    pub rd: u5,
    pub funct3: u3,
    pub rs1: u5,
    pub rs2: u5,
    pub funct7: u7,
}

impl From<u32> for RTypeInstructionFormat {
    fn from(value: u32) -> Self {
        RTypeInstructionFormat {
            opcode: value.opcode(),
            funct7: value.funct7(),
            funct3: value.funct3(),
            rd: value.rd(),
            rs1: value.rs1(),
            rs2: value.rs2(),
        }
    }
}

impl From<RTypeInstructionFormat> for u32 {
    fn from(value: RTypeInstructionFormat) -> Self {
        0.with_opcode(value.opcode)
            .with_rd(value.rd)
            .with_funct3(value.funct3)
            .with_funct7(value.funct7)
            .with_rs1(value.rs1)
            .with_rs2(value.rs2)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct STypeSImmediateInstruction {
    pub opcode: u7,
    pub funct3: u3,
    pub rs1: u5,
    pub rs2: u5,
    pub imm: u32,
}

impl From<u32> for STypeSImmediateInstruction {
    fn from(value: u32) -> Self {
        Self {
            opcode: value.opcode(),
            funct3: value.funct3(),
            rs1: value.rs1(),
            rs2: value.rs2(),
            imm: value.immediate(&RISCVImmediate::S),
        }
    }
}

impl From<STypeSImmediateInstruction> for u32 {
    fn from(value: STypeSImmediateInstruction) -> Self {
        0.with_opcode(value.opcode)
            .with_funct3(value.funct3)
            .with_rs1(value.rs1)
            .with_rs2(value.rs2)
            .with_immediate(value.imm, &RISCVImmediate::S)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct STypeBImmediateInstruction {
    pub opcode: u7,
    pub funct3: u3,
    pub rs1: u5,
    pub rs2: u5,
    pub imm: u32,
}

impl From<u32> for STypeBImmediateInstruction {
    fn from(value: u32) -> Self {
        Self {
            opcode: value.opcode(),
            funct3: value.funct3(),
            rs1: value.rs1(),
            rs2: value.rs2(),
            imm: value.immediate(&RISCVImmediate::B),
        }
    }
}

impl From<STypeBImmediateInstruction> for u32 {
    fn from(value: STypeBImmediateInstruction) -> Self {
        0.with_opcode(value.opcode)
            .with_funct3(value.funct3)
            .with_rs1(value.rs1)
            .with_rs2(value.rs2)
            .with_immediate(value.imm, &RISCVImmediate::B)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ITypeIImmediateInstruction {
    pub opcode: u7,
    pub rd: u5,
    pub funct3: u3,
    pub rs1: u5,
    pub imm: u32,
}

impl From<u32> for ITypeIImmediateInstruction {
    fn from(value: u32) -> Self {
        Self {
            opcode: value.opcode(),
            rd: value.rd(),
            funct3: value.funct3(),
            rs1: value.rs1(),
            imm: value.immediate(&RISCVImmediate::I),
        }
    }
}

impl From<ITypeIImmediateInstruction> for u32 {
    fn from(value: ITypeIImmediateInstruction) -> Self {
        0.with_opcode(value.opcode)
            .with_rd(value.rd)
            .with_funct3(value.funct3)
            .with_rs1(value.rs1)
            .with_immediate(value.imm, &RISCVImmediate::I)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct UTypeJImmediateInstruction {
    pub opcode: u7,
    pub rd: u5,
    pub imm: u32,
}

impl From<u32> for UTypeJImmediateInstruction {
    fn from(value: u32) -> Self {
        Self {
            opcode: value.opcode(),
            rd: value.rd(),
            imm: value.immediate(&RISCVImmediate::J),
        }
    }
}

impl From<UTypeJImmediateInstruction> for u32 {
    fn from(value: UTypeJImmediateInstruction) -> Self {
        0.with_opcode(value.opcode)
            .with_rd(value.rd)
            .with_immediate(value.imm, &RISCVImmediate::J)
    }
}
