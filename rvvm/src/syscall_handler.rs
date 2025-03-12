use std::fmt::{Display, Formatter, Result};

use super::Cpu;
use super::Generic;
use super::OperationError;
use super::cpu::Gsr;
use super::syscall::Sysno;
use colored::Colorize;
pub struct SyscallArgs {
    no: Sysno,
    args: [usize; 7],
}
impl Display for SyscallArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let _ = write!(f, "{}", self.no);
        for n in 0..self.args.len() {
            let _ = write!(f, ", a{}: {:#x}", n, self.arg(n));
        }
        Ok(())
    }
}
impl SyscallArgs {
    pub fn from_register(reg: &Gsr) -> Self {
        SyscallArgs {
            no: Sysno::from(reg.get(Generic::a7) as u64),
            args: [
                reg.get(Generic::a0),
                reg.get(Generic::a1),
                reg.get(Generic::a2),
                reg.get(Generic::a3),
                reg.get(Generic::a4),
                reg.get(Generic::a5),
                reg.get(Generic::a6),
            ],
        }
    }
    pub fn arg(&self, n: usize) -> usize {
        self.args[n]
    }
}
pub fn syscall_handler(cpu: &mut Cpu) -> anyhow::Result<(), OperationError> {
    let reg = &mut cpu.x;
    let mem = &mut cpu.mem;
    let syscall = SyscallArgs::from_register(reg);
    println!("{}", format!("Syscall: {}", syscall.no).blue().bold());
    match syscall.no {
        Sysno::write => {
            let data = mem.read_bytes(syscall.arg(1), syscall.arg(2))?;
            println!(
                "    {}",
                format!("{:?}", String::from_utf8_lossy(data)).green()
            );
            reg.set(Generic::a0, 0);
        }
        Sysno::exit => {
            println!(
                "    {}",
                match syscall.arg(1) {
                    0 => format!("exit({})", syscall.arg(0)).green(),
                    _ => format!("exit({})", syscall.arg(0)).red(),
                }
            );
            cpu.running = false;
        }
        _ => {}
    }
    Ok(())
}
