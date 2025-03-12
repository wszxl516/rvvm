use super::bus::{Bus, BusOperation};
use super::error::OperationError;
use core::ops::RangeInclusive;
pub struct Memory {
    data: Vec<u8>,
    range: RangeInclusive<usize>,
}
impl Memory {
    pub fn new(range: RangeInclusive<usize>) -> Self {
        Self {
            data: vec![0u8; (range.end() - range.start()) as usize],
            range,
        }
    }
    #[inline(always)]
    pub fn get_address(&self, addr: usize) -> anyhow::Result<usize, OperationError> {
        if self.range.contains(&addr) {
            Ok((addr - self.range.start()) as usize)
        } else {
            Err(OperationError::AddressOutOfRange(addr))
        }
    }
    fn load_interger<T: Sized>(&self, addr: usize) -> anyhow::Result<T, OperationError> {
        if addr % size_of::<T>() != 0 {
            return Err(OperationError::UnalignedAccess(addr));
        }
        if self.get_address(addr).is_err() || self.get_address(addr + size_of::<T>() - 1).is_err() {
            return Err(OperationError::LoadAddressFault(addr));
        }
        Ok(unsafe { (self.data.as_ptr().add(addr as usize) as *const T).read() })
    }
    fn store_interger<T: Sized>(
        &mut self,
        addr: usize,
        data: T,
    ) -> anyhow::Result<(), OperationError> {
        if addr % size_of::<T>() != 0 {
            return Err(OperationError::UnalignedAccess(addr));
        }
        if self.get_address(addr).is_err() || self.get_address(addr + size_of::<T>() - 1).is_err() {
            return Err(OperationError::StoreAddressFault(addr));
        }
        Ok(unsafe { (self.data.as_ptr().add(addr as usize) as *mut T).write(data) })
    }
}
impl Bus for Memory {
    fn init_from(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.data[0..data.len()].copy_from_slice(data);
        Ok(())
    }
    fn address_range(&self) -> &RangeInclusive<usize> {
        &self.range
    }
    fn read_bytes(&self, addr: usize, len: usize) -> anyhow::Result<&[u8], OperationError> {
        let start = self
            .get_address(addr)
            .map_err(|_| OperationError::LoadAddressFault(addr))?;
        let end = self
            .get_address(addr + len)
            .map_err(|_| OperationError::LoadAddressFault(addr + len))?;
        Ok(&self.data[start..end])
    }
    fn write_bytes(&mut self, addr: usize, data: &[u8]) -> anyhow::Result<(), OperationError> {
        let len = data.len();
        let start = self
            .get_address(addr)
            .map_err(|_| OperationError::StoreAddressFault(addr))?;
        let end = self
            .get_address(addr + len)
            .map_err(|_| OperationError::StoreAddressFault(addr + len))?;
        self.data[start..end].copy_from_slice(data);
        Ok(())
    }
}
impl BusOperation<u8> for Memory {
    fn load(&self, addr: usize) -> anyhow::Result<u8, OperationError> {
        self.load_interger(addr)
    }
    fn store(&mut self, addr: usize, data: u8) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}

impl BusOperation<u16> for Memory {
    fn load(&self, addr: usize) -> anyhow::Result<u16, OperationError> {
        self.load_interger(addr)
    }
    fn store(&mut self, addr: usize, data: u16) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}
impl BusOperation<u32> for Memory {
    fn load(&self, addr: usize) -> anyhow::Result<u32, OperationError> {
        self.load_interger(addr)
    }
    fn store(&mut self, addr: usize, data: u32) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}

impl BusOperation<u64> for Memory {
    fn load(&self, addr: usize) -> anyhow::Result<u64, OperationError> {
        self.load_interger(addr)
    }
    fn store(&mut self, addr: usize, data: u64) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}

impl BusOperation<usize> for Memory {
    fn load(&self, addr: usize) -> anyhow::Result<usize, OperationError> {
        self.load_interger(addr)
    }
    fn store(&mut self, addr: usize, data: usize) -> anyhow::Result<(), OperationError> {
        self.store_interger(addr, data)
    }
}
