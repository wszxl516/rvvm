#![allow(dead_code)]
#![allow(unused)]

use super::bus::Bus;
use super::error::OperationError;
use super::register::Register;
use super::syscall_handler::syscall_handler;
use crate::register::Generic;
use colored::Colorize;
use riscv::Op;
pub type Gsr = Register<usize, 31>;
pub type Fsr = Register<isize, 31>;

pub struct Cpu {
    pub x: Gsr,
    pub f: Fsr,
    pub mem: Box<dyn Bus>,
    pc: usize,
    is_debug: bool,
    pub running: bool,
}
impl Cpu {
    pub fn new(mem: impl Bus + 'static) -> Self {
        Self {
            x: Gsr::new(),
            f: Fsr::new(),
            mem: Box::new(mem),
            pc: 0,
            is_debug: false,
            running: false,
        }
    }
    fn fetch_instruction(&mut self) -> anyhow::Result<(Op, u64, u32), OperationError> {
        let bits: u16 = self.mem.load(self.pc)?;
        if bits & 3 == 3 {
            // The instruction will cross page boundary.
            if self.pc & 4095 == 4094 {
                return Err(OperationError::IllegalInstruction(bits as _, self.pc));
            }
            let hi_bits: u16 = self.mem.load(self.pc + 2)?;
            let bits = (hi_bits as u32) << 16 | bits as u32;
            let op = riscv::decode(bits);
            Ok((op, 4, bits))
        } else {
            let op = riscv::decode_compressed(bits);
            Ok((op, 2, bits as u32))
        }
    }
    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.is_debug = debug
    }
    pub fn tick(&mut self) -> anyhow::Result<(), OperationError> {
        match self.fetch_instruction() {
            Ok((op, len, bits)) => match op {
                Op::Illegal => return Err(OperationError::IllegalInstruction(bits, self.pc)),
                _ => {
                    if self.is_debug {
                        println!("{}", op.pretty_print(self.pc as u64, bits));
                    }
                    instruction_operation(op, self)?;
                    self.pc += len as usize;
                }
            },
            Err(ee) => return Err(ee),
        }
        Ok(())
    }
    pub fn run(&mut self) {
        self.running = true;
        loop {
            if !self.running {
                return;
            }
            match self.tick() {
                Ok(_) => {}
                Err(err) => {
                    println!("{}", err.to_string().red());
                    break;
                }
            }
        }
    }
}

pub fn instruction_operation(op: Op, cpu: &mut Cpu) -> anyhow::Result<(), OperationError> {
    match op {
        Op::Addi { rd, rs1, imm } => {
            let src = cpu.x.get(Generic::from(rs1));
            cpu.x.set(Generic::from(rd), src + imm as usize);
        }
        Op::Auipc { rd, imm } => {
            let address = cpu.pc as i64;
            cpu.x
                .set(Generic::from(rd), address.wrapping_add(imm as i64) as usize);
        }
        Op::Ld { rd, rs1, imm } => {
            let src = cpu.x.get(Generic::from(rs1)) as isize + imm as isize;
            let address: usize = cpu.mem.load(src as usize)?;
            cpu.x.set(Generic::from(rd), address);
        }
        Op::Ecall => {
            syscall_handler(cpu)?;
        }
        _ => {}
    }
    Ok(())
}
