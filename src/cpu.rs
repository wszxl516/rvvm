#![allow(dead_code)]
#![allow(unused)]

use crate::register::Generic;

use super::bus::Bus;
use super::error::OperationError;
use super::register::Register;
use colored::Colorize;
use riscv::Op;
pub type Gsr = Register<u64, 31>;
pub type Fsr = Register<i64, 31>;

pub struct Cpu {
    x: Gsr,
    f: Fsr,
    mem: Box<dyn Bus>,
    pc: u64,
    is_debug: bool,
}
impl Cpu {
    pub fn new(mem: impl Bus + 'static) -> Self {
        Self {
            x: Gsr::new(),
            f: Fsr::new(),
            mem: Box::new(mem),
            pc: 0,
            is_debug: false,
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
    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc
    }
    pub fn set_debug(&mut self, debug: bool) {
        self.is_debug = debug
    }
    pub fn tick(&mut self) -> anyhow::Result<(), OperationError> {
        match self.fetch_instruction() {
            Ok((op,len, bits)) => match op {
                Op::Illegal => return Err(OperationError::IllegalInstruction(bits, self.pc)),
                _ => {
                    instruction_operation(op, &mut self.x, &mut self.pc, &mut self.mem)?;
                    if self.is_debug {
                        println!("{}", op.pretty_print(self.pc, bits));
                    }
                    self.pc += len;
                }
            },
            Err(ee) => return Err(ee),
        }
        Ok(())
    }
    pub fn run(&mut self) {
        loop {
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

pub fn instruction_operation(
    op: Op,
    reg: &mut Gsr,
    pc: &mut u64,
    mem: &mut Box<dyn Bus>,
) -> anyhow::Result<(), OperationError> {
    match op {
        Op::Addi { rd, rs1, imm } => {
            let src = reg.get(Generic::from(rs1));
            reg.set(Generic::from(rd), src + imm as u64);
        }
        Op::Auipc { rd, imm } => {
            let address = *pc as i64;
            reg.set(Generic::from(rd), address.wrapping_add(imm as i64) as u64);
        }
        Op::Ld { rd, rs1, imm } => {
            let src = reg.get(Generic::from(rs1)) as i64 + imm as i64;
            let address: u64 = mem.load(src as u64)?;
            reg.set(Generic::from(rd), address);
        }
        Op::Ecall => {
            ecall(reg, mem)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn ecall(
    reg: &mut Gsr,
    mem: &mut Box<dyn Bus>,
) -> anyhow::Result<(), OperationError> {
    let a0 = reg.get(Generic::a0);
    let a1 = reg.get(Generic::a1);
    let a2 = reg.get(Generic::a2);
    let a3 = reg.get(Generic::a3);
    let a4 = reg.get(Generic::a4);
    let a5 = reg.get(Generic::a5);
    let a6 = reg.get(Generic::a6);
    let a7 = reg.get(Generic::a7);
    println!(
        "{}, {}",
        format!("Syscall: {}", super::syscall::Sysno::from(a7))
            .blue()
            .bold(),
            format!("a0: {:#x}, a1: {:#x}, a2: {:#x}, a3: {:#x}, a4: {:#x}, a5: {:#x}, a6: {:#x}", a0, a1, a2, a3, a4, a5, a6).green()
    );
    match a7 {
        64 => {
            let data = mem.read_bytes(a1, a2)?;
            println!(
                "    {}",
                format!("{:?}", String::from_utf8_lossy(data)).green()
            )
        }
        93 => {
            println!("    {}", match a0 {
                0 => format!("exit({})", a0).green(),
                _ => format!("exit({})", a0).red(),
            })
        }
        _ => {}
    }
    Ok(())
}
