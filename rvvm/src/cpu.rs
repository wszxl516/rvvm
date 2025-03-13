#![allow(dead_code)]
#![allow(unused)]

use super::bus::Bus;
use super::error::OperationError;
use super::operation::instruction_operation;
use super::register::Register;
use crate::register::{Float, Generic};
use colored::Colorize;
use riscv::Op;
pub type Gsr = Register<isize, 32>;
pub type Fsr = Register<isize, 32>;

pub struct Cpu {
    generic: Gsr,
    float: Fsr,
    pub mem: Box<dyn Bus>,
    pub pc: isize,
    is_debug: bool,
    pub running: bool,
}
impl Cpu {
    pub fn new(mem: impl Bus + 'static) -> Self {
        Self {
            generic: Gsr::new(),
            float: Fsr::new(),
            mem: Box::new(mem),
            pc: 0,
            is_debug: false,
            running: false,
        }
    }
    fn fetch_instruction(&mut self) -> anyhow::Result<(Op, u64, u32), OperationError> {
        let bits: u16 = self.mem.load(self.pc as usize)?;
        if bits & 3 == 3 {
            // The instruction will cross page boundary.
            if self.pc & 4095 == 4094 {
                return Err(OperationError::IllegalInstruction(
                    bits as _,
                    self.pc as usize,
                ));
            }
            let hi_bits: u16 = self.mem.load((self.pc + 2) as usize)?;
            let bits = (hi_bits as u32) << 16 | bits as u32;
            let op = riscv::decode(bits);
            Ok((op, 4, bits))
        } else {
            let op = riscv::decode_compressed(bits);
            Ok((op, 2, bits as u32))
        }
    }
    pub fn set_pc(&mut self, pc: isize) {
        self.pc = pc
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.is_debug = debug
    }
    pub fn tick(&mut self) -> anyhow::Result<(), OperationError> {
        match self.fetch_instruction() {
            Ok((op, len, bits)) => match op {
                Op::Illegal => {
                    return Err(OperationError::IllegalInstruction(bits, self.pc as usize));
                }
                _ => {
                    if self.is_debug {
                        println!("{}", op.pretty_print(self.pc as u64, bits));
                    }
                    instruction_operation(op, self, len as isize)?;
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
    #[inline]
    pub fn set_generic(&mut self, name: Generic, value: isize) {
        if name != Generic::zero {
            self.generic.set(name, value);
        }
    }
    #[inline]
    pub fn get_generic(&self, name: Generic) -> isize {
        if name != Generic::zero {
            self.generic.get(name)
        } else {
            0
        }
    }
    #[inline]
    pub fn set_float(&mut self, name: Float, value: isize) {
        self.float.set(name, value);
    }
    #[inline]
    pub fn get_float(&self, name: Float) -> isize {
        self.float.get(name)
    }
}
