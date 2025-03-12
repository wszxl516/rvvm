use std::fmt::{Display, Formatter, Result};

use super::Cpu;
use super::Generic;
use super::OperationError;
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
    pub fn from_register(cpu: &Cpu) -> Self {
        SyscallArgs {
            no: Sysno::from(cpu.get_generic(Generic::a7) as u64),
            args: [
                cpu.get_generic(Generic::a0) as usize,
                cpu.get_generic(Generic::a1) as usize,
                cpu.get_generic(Generic::a2) as usize,
                cpu.get_generic(Generic::a3) as usize,
                cpu.get_generic(Generic::a4) as usize,
                cpu.get_generic(Generic::a5) as usize,
                cpu.get_generic(Generic::a6) as usize,
            ],
        }
    }
    pub fn arg(&self, n: usize) -> usize {
        self.args[n]
    }
}
pub fn syscall_handler(cpu: &mut Cpu) -> anyhow::Result<(), OperationError> {
    let syscall = SyscallArgs::from_register(cpu);
    let mem = &mut cpu.mem;
    println!("{}", format!("Syscall: {}", syscall.no).blue().bold());
    match syscall.no {
        Sysno::write => {
            let data = mem.read_bytes(syscall.arg(1), syscall.arg(2))?;
            println!(
                "    {}",
                format!("{:?}", String::from_utf8_lossy(data)).green()
            );
            cpu.set_generic(Generic::a0, 0);
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
