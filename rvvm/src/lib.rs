#![feature(new_range_api)]
#![feature(adt_const_params)]
mod bus;
mod cpu;
mod error;
mod macros;
mod memory;
mod register;
mod syscall;
mod syscall_handler;
mod operation;
pub use bus::{Bus, BusOperation};
pub use cpu::Cpu;
pub use error::OperationError;
pub use memory::Memory;
pub use register::{Generic, Register};
pub use syscall::Sysno;
