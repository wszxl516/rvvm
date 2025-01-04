use core::ops::RangeInclusive;
use super::error::OperationError;
use super::bus::{Bus, BusOperation};
pub struct Memory{
    data: Vec<u8>,
    range: RangeInclusive<u64>
}
impl Memory {
    pub fn new(range: RangeInclusive<u64>) -> Self {
        Self{data: vec![0u8; (range.end() - range.start()) as usize], range}
    }
    #[inline(always)]
    pub fn get_address(&self, addr: u64) -> anyhow::Result<usize, OperationError> {
        if self.range.contains(&addr){
            Ok((addr - self.range.start())  as usize)
        }
        else {
            Err(OperationError::AddressOutOfRange(addr))
        }
    }
    fn load_interger<T:Sized>(&self, addr: u64) -> anyhow::Result<T, OperationError> {
        if self.get_address(addr).is_err() || self.get_address(addr+ size_of::<T>() as u64  - 1 ).is_err(){
            return Err(OperationError::LoadAddressFault(addr))
        }
        Ok(unsafe { (self.data.as_ptr().add(addr as usize) as *const T).read() })
    }
    fn store_interger<T:Sized>(&mut self, addr: u64, data: T) -> anyhow::Result<(), OperationError> {
        if self.get_address(addr).is_err() || self.get_address(addr+ size_of::<T>() as u64  - 1 ).is_err(){
            return Err(OperationError::StoreAddressFault(addr))
        }
        Ok(unsafe { (self.data.as_ptr().add(addr as usize) as *mut T).write(data) })
    }
}
impl Bus for Memory {
    fn init_from(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.data[0..data.len()].copy_from_slice(data);
        Ok(())
    }
    fn address_range(&self) -> &RangeInclusive<u64>{
        &self.range
    }
    fn read_bytes(&self, addr: u64, len: u64) -> anyhow::Result<&[u8], OperationError> {
        let start = self.get_address(addr).map_err(|_|OperationError::LoadAddressFault(addr))?;
        let end = self.get_address(addr + len).map_err(|_|OperationError::LoadAddressFault(addr + len))?;
        Ok(&self.data[start..end])
    }
    fn write_bytes(&mut self, addr: u64, data: &[u8]) -> anyhow::Result<(), OperationError>{
        let len = data.len() as u64;
        let start = self.get_address(addr).map_err(|_|OperationError::StoreAddressFault(addr))?;
        let end = self.get_address(addr + len).map_err(|_|OperationError::StoreAddressFault(addr + len))?;
        self.data[start.. end].copy_from_slice(data);
        Ok(())
    }

}
impl BusOperation<u8> for Memory {
    fn load(&self, addr: u64) -> anyhow::Result<u8, OperationError> {
        self.load_interger(addr)
    }
    fn store(&mut self, addr: u64, data: u8) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}

impl BusOperation<u16> for Memory {
    fn load(&self, addr: u64) -> anyhow::Result<u16, OperationError> {
        self.load_interger(addr)

    }
    fn store(&mut self, addr: u64, data: u16) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}
impl BusOperation<u32> for Memory {
    fn load(&self, addr: u64) -> anyhow::Result<u32, OperationError> {
        self.load_interger(addr)

    }
    fn store(&mut self, addr: u64, data: u32) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}

impl BusOperation<u64> for Memory {
    fn load(&self, addr: u64) -> anyhow::Result<u64, OperationError> {
        self.load_interger(addr)

    }
    fn store(&mut self, addr: u64, data: u64) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}