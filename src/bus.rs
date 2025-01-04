use core::ops::RangeInclusive;
use super::error::OperationError;


pub trait Bus: BusOperation<u8> + BusOperation<u16> + BusOperation<u32> + BusOperation<u64> {
    fn init_from(&mut self, data: &[u8]) -> anyhow::Result<()>;
    fn address_range(&self) -> &RangeInclusive<u64>;
    fn read_bytes(&self, addr: u64, len: u64) -> anyhow::Result<&[u8], OperationError>;
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> anyhow::Result<(), OperationError>;

}
pub trait BusOperation<T:Sized> {
    fn load(&self, addr: u64) -> anyhow::Result<T, OperationError>;
    fn store(&mut self, addr: u64, data: T) -> anyhow::Result<(), OperationError>;
}