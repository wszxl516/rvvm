use super::cpu::Cpu;
use super::error::OperationError;
use super::syscall_handler::syscall_handler;
use crate::register::Generic;
use riscv::Op;

pub fn instruction_operation(
    op: Op,
    cpu: &mut Cpu,
    len: isize,
) -> anyhow::Result<(), OperationError> {
    match op {
        Op::Add { rd, rs1, rs2 } => {
            cpu.set_generic(
                Generic::from(rd),
                cpu.get_generic(Generic::from(rs1)) + cpu.get_generic(Generic::from(rs2)),
            );
        }

        Op::Sub { rd, rs1, rs2 } => {
            cpu.set_generic(
                Generic::from(rd),
                cpu.get_generic(Generic::from(rs1)) - cpu.get_generic(Generic::from(rs2)),
            );
        }
        Op::Slli { rd, rs1, imm } => {
            cpu.set_generic(
                Generic::from(rd),
                cpu.get_generic(Generic::from(rs1)) << imm,
            );
        }
        Op::Addi { rd, rs1, imm } => {
            let src = cpu.get_generic(Generic::from(rs1));
            cpu.set_generic(Generic::from(rd), src + imm as isize);
        }
        Op::Auipc { rd, imm } => {
            cpu.set_generic(Generic::from(rd), cpu.pc.wrapping_add(imm as isize));
        }
        Op::Lb { rd, rs1, imm } => {
            let src = cpu
                .get_generic(Generic::from(rs1))
                .wrapping_add(imm as isize);
            let data: u8 = cpu.mem.load(src as usize)?;
            cpu.set_generic(Generic::from(rd), data as isize);
        }
        Op::Ld { rd, rs1, imm } => {
            let src = cpu
                .get_generic(Generic::from(rs1))
                .wrapping_add(imm as isize);
            let address: usize = cpu.mem.load(src as usize)?;
            cpu.set_generic(Generic::from(rd), address as isize);
        }
        Op::Jal { rd, imm } => {
            cpu.set_generic(Generic::from(rd), cpu.pc + 4);
            cpu.pc += imm as isize;
            return Ok(());
        }
        Op::Jalr { rd, rs1, imm } => {
            cpu.set_generic(Generic::from(rd), cpu.pc + 4);
            cpu.pc = cpu.get_generic(Generic::from(rs1)) + imm as isize;
            return Ok(());
        }
        Op::Bne { rs1, rs2, imm } => {
            if cpu.get_generic(Generic::from(rs1)) != cpu.get_generic(Generic::from(rs2)) {
                cpu.pc += imm as isize;
            } else {
                cpu.pc += len;
            }
            return Ok(());
        }

        Op::Ecall => {
            syscall_handler(cpu)?;
        }
        _ => {
            panic!("op: {:?} Not implemented!", op);
        }
    }
    cpu.pc += len;
    Ok(())
}
