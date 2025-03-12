#![feature(new_range_api)]
use rvvm::{Bus, Cpu, Memory};
use std::io::Read;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value = "code.bin")]
    name: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 0)]
    offset: usize,
    /// Number of times to greet
    #[arg(short, long, action, default_value_t = false)]
    verbose: bool,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut buffer = Vec::new();
    std::fs::File::open(&args.name)
        .map_err(|err| anyhow::anyhow!("{} {}", err, args.name))?
        .read_to_end(&mut buffer)?;
    let mut mem = Memory::new(0usize..=buffer.len());
    mem.init_from(&buffer)?;
    let mut c = Cpu::new(mem);
    c.set_pc(args.offset);
    c.set_debug(args.verbose);
    c.run();
    Ok(())
}
