use super::error::OperationError;
use core::ops::RangeInclusive;

pub trait Bus:
    BusOperation<u8> + BusOperation<u16> + BusOperation<u32> + BusOperation<u64> + BusOperation<usize>
{
    fn init_from(&mut self, data: &[u8]) -> anyhow::Result<()>;
    fn address_range(&self) -> &RangeInclusive<usize>;
    fn read_bytes(&self, addr: usize, len: usize) -> anyhow::Result<&[u8], OperationError>;
    fn write_bytes(&mut self, addr: usize, data: &[u8]) -> anyhow::Result<(), OperationError>;
}
pub trait BusOperation<T: Sized> {
    fn load(&self, addr: usize) -> anyhow::Result<T, OperationError>;
    fn store(&mut self, addr: usize, data: T) -> anyhow::Result<(), OperationError>;
}
