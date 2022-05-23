pub mod risc_machine;
pub mod riscv_instruction;

#[cfg(test)]
mod tests {

    use rand::Rng;
    use ux::{u20, u5};

    use crate::risc_machine::{CPUInstruction, Memory, RISCMachine, Registry};
    use crate::riscv_instruction::{RISCVImmediate, RISCVInstruction};

    #[test]
    fn it_correctly_encodes_immediate() {
        for encoding in [RISCVImmediate::I, RISCVImmediate::S, RISCVImmediate::J] {
            let imm = 0b1010100;
            let a: u32 = 0.with_immediate(0b1010100, &encoding);
            let result = a.immediate(&encoding);
            assert_eq!(
                imm, result,
                "Incorrect encoding (provided: {:#034b}, decoded: {:#034b})",
                imm, result
            )
        }
    }

    #[test]
    fn it_correctly_decodes_add() -> Result<(), ()> {
        let add = CPUInstruction::ADD {
            rd: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            rs1: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            rs2: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
        };

        let encoded: u32 = add.into();
        let decoded: CPUInstruction = encoded.try_into()?;
        assert_eq!(add, decoded);

        Ok(())
    }
    #[test]
    fn it_correctly_decodes_sub() -> Result<(), ()> {
        let sub = CPUInstruction::SUB {
            rd: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            rs1: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            rs2: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
        };

        let encoded: u32 = sub.into();
        let decoded: CPUInstruction = encoded.try_into()?;
        assert_eq!(sub, decoded);

        Ok(())
    }

    #[test]
    fn it_correctly_decodes_lw() -> Result<(), ()> {
        let load = CPUInstruction::LW {
            rd: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            rs1: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            imm: 1,
        };

        let encoded: u32 = load.into();
        let decoded: CPUInstruction = encoded.try_into()?;
        assert_eq!(load, decoded);

        Ok(())
    }
    #[test]
    fn it_correctly_decodes_sw() -> Result<(), ()> {
        let sw = CPUInstruction::SW {
            rs1: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            rs2: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            imm: 1,
        };

        let encoded: u32 = sw.into();
        let decoded: CPUInstruction = encoded.try_into()?;
        assert_eq!(sw, decoded);

        Ok(())
    }
    #[test]
    fn it_correctly_decodes_jal() -> Result<(), ()> {
        let jal = CPUInstruction::JAL {
            rd: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            imm: 0b11111111111100000000000000000000,
        };

        let encoded: u32 = jal.into();
        let decoded: CPUInstruction = encoded.try_into()?;
        assert_eq!(jal, decoded);
        Ok(())
    }
    #[test]
    fn it_correctly_decodes_jalr() -> Result<(), ()> {
        let sw = CPUInstruction::JALR {
            rd: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            rs1: u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into())),
            imm: 1,
        };

        let encoded: u32 = sw.into();
        let decoded: CPUInstruction = encoded.try_into()?;
        assert_eq!(sw, decoded);
        Ok(())
    }

    #[test]
    fn it_can_load_from_memory() -> Result<(), ()> {
        let mut memory = Memory::default();
        let rd = u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into()));
        let value_to_load: u32 = rand::thread_rng().gen();
        let load_instruction = CPUInstruction::LW {
            rd,
            rs1: u5::new(0),
            imm: 4,
        };

        memory.set_four_byte(u20::new(0), load_instruction.into());
        memory.set_four_byte(u20::new(4), value_to_load);
        let mut machine = RISCMachine::default();
        machine.load_memory(memory);
        machine.tick()?;
        assert_eq!(value_to_load, machine.registry.get(rd));
        Ok(())
    }

    #[test]
    fn it_can_save_to_memory() -> Result<(), ()> {
        let index_to_save = u20::new(rand::thread_rng().gen_range(0..=u20::MAX.into()));
        let rs1 = u5::new(rand::thread_rng().gen_range(2..=u5::MAX.into()));
        let value_to_save: u32 = rand::thread_rng().gen();
        let rs2 = u5::new(1);

        let save_instruction = CPUInstruction::SW { rs1, rs2, imm: 0 };

        let mut memory = Memory::default();
        memory.set_four_byte(u20::new(0), save_instruction.into());
        let mut registry = Registry::default();
        registry.set(rs1, index_to_save.into());
        registry.set(rs2, value_to_save);

        let mut machine = RISCMachine {
            memory,
            registry,
            program_counter: u20::new(0),
        };
        machine.tick()?;
        assert_eq!(value_to_save, machine.memory.get_aligned(index_to_save));
        Ok(())
    }
    #[test]
    fn it_can_add() -> Result<(), ()> {
        let rd = u5::new(1);
        let add1: u32 = rand::thread_rng().gen();
        let rs1 = u5::new(2);
        let add2: u32 = rand::thread_rng().gen();
        let rs2 = u5::new(rand::thread_rng().gen_range(3..=u5::MAX.into()));

        let instruction = CPUInstruction::ADD { rd, rs1, rs2 };

        let mut memory = Memory::default();
        memory.set_four_byte(u20::new(0), instruction.into());
        let mut registry = Registry::default();
        registry.set(rs1, add1);
        registry.set(rs2, add2);

        let mut machine = RISCMachine {
            memory,
            registry,
            program_counter: u20::new(0),
        };
        machine.tick()?;
        assert_eq!(add1.saturating_add(add2), machine.registry.get(rd));
        Ok(())
    }
    #[test]
    fn it_can_subtract() -> Result<(), ()> {
        let rd = u5::new(1);
        let lhs: u32 = rand::thread_rng().gen();
        let rs1 = u5::new(2);
        let rhs: u32 = rand::thread_rng().gen();
        let rs2 = u5::new(rand::thread_rng().gen_range(3..=u5::MAX.into()));

        let instruction = CPUInstruction::SUB { rd, rs1, rs2 };

        let mut memory = Memory::default();
        memory.set_four_byte(u20::new(0), instruction.into());
        let mut registry = Registry::default();
        registry.set(rs1, lhs);
        registry.set(rs2, rhs);

        let mut machine = RISCMachine {
            memory,
            registry,
            program_counter: u20::new(0),
        };
        machine.tick()?;
        assert_eq!(lhs.saturating_sub(rhs), machine.registry.get(rd));
        Ok(())
    }
    #[test]
    fn it_can_jal() -> Result<(), ()> {
        let start = u20::new(rand::thread_rng().gen_range(0..=u20::MAX.into()) & 0b11111110);
        let end = u20::new(rand::thread_rng().gen_range(0..=u20::MAX.into()) & 0b11111110);
        let relative = u32::from(end) as i32 - u32::from(start) as i32;

        let rd = u5::new(rand::thread_rng().gen_range(1..=u5::MAX.into()));

        let instruction = CPUInstruction::JAL {
            rd,
            imm: relative as u32,
        };

        let mut memory = Memory::default();
        memory.set_four_byte(start, instruction.into());

        let mut machine = RISCMachine {
            memory,
            registry: Registry::default(),
            program_counter: start,
        };

        machine.tick()?;
        assert_eq!(end, machine.program_counter);
        assert_eq!(u32::from(start) + 4, machine.registry.get(rd));
        Ok(())
    }

    fn fibonacci(n: u32) -> u32 {
        match n {
            0 => 1,
            1 => 1,
            _ => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    #[test]
    fn it_can_fibonacci() -> Result<(), ()> {
        let x0 = u5::new(0);
        // set to constant 1
        let x1 = u5::new(1);

        //temporary storage
        let x4 = u5::new(4);
        //registry to store fib(n)
        let x5 = u5::new(5);
        //registry to store fib(n+1)
        let x6 = u5::new(6);
        //registry to store n - num_loops
        let x7 = u5::new(7);
        let halt_position = u20::new(10 * 4);

        let mut memory = Memory::default();
        // set fib(0) to 1
        memory.set_four_byte(
            u20::new(0),
            CPUInstruction::LW {
                rd: x5,
                rs1: x0,
                imm: 11 * 4,
            }
            .into(),
        );
        // set fib(1) to 1
        memory.set_four_byte(
            u20::new(4),
            CPUInstruction::LW {
                rd: x6,
                rs1: x0,
                imm: 11 * 4,
            }
            .into(),
        );
        // set n - num_loops to n
        memory.set_four_byte(
            u20::new(2 * 4),
            CPUInstruction::LW {
                rd: x7,
                rs1: x0,
                imm: 12 * 4,
            }
            .into(),
        );
        // set x1 to 1
        memory.set_four_byte(
            u20::new(3 * 4),
            CPUInstruction::LW {
                rd: x1,
                rs1: x0,
                imm: 11 * 4,
            }
            .into(),
        );
        //If n - num_loops == 0 exit
        memory.set_four_byte(
            u20::new(4 * 4),
            CPUInstruction::BEQ {
                rs1: x0,
                rs2: x7,
                imm: u32::from(halt_position - u20::new(4 * 4)),
            }
            .into(),
        );

        //add fib(n) + fib(n-1) and store in x4
        memory.set_four_byte(
            u20::new(5 * 4),
            CPUInstruction::ADD {
                rs1: x5,
                rs2: x6,
                rd: x4,
            }
            .into(),
        );
        //shuffle memory
        memory.set_four_byte(
            u20::new(6 * 4),
            CPUInstruction::ADD {
                rs1: x0,
                rs2: x6,
                rd: x5,
            }
            .into(),
        );
        memory.set_four_byte(
            u20::new(7 * 4),
            CPUInstruction::ADD {
                rs1: x0,
                rs2: x4,
                rd: x6,
            }
            .into(),
        );
        //decrement counter
        memory.set_four_byte(
            u20::new(8 * 4),
            CPUInstruction::SUB {
                rs1: x7,
                rs2: x1,
                rd: x7,
            }
            .into(),
        );
        //jump to breakpoint
        memory.set_four_byte(
            u20::new(9 * 4),
            CPUInstruction::JAL {
                rd: x0,
                imm: -((5 * 4) as i32) as u32,
            }
            .into(),
        );

        //program data
        memory.set_four_byte(u20::new(11 * 4), 1);
        let n = rand::thread_rng().gen_range(0..10);
        memory.set_four_byte(u20::new(12 * 4), n);

        let mut machine = RISCMachine::default();
        machine.load_memory(memory);
        machine.run(&|pc| -> bool { pc == halt_position })?;
        assert_eq!(fibonacci(n), machine.registry.get(x5));
        Ok(())
    }
}
