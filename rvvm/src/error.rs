use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("store address fault from `{0:#X}`")]
    StoreAddressFault(usize),
    #[error("load  address fault from `{0:#X}`")]
    LoadAddressFault(usize),
    #[error("load or store address unaligned from `{0:#X}`")]
    UnalignedAccess(usize),
    #[error("address `{0:#X}` out of range")]
    AddressOutOfRange(usize),
    #[error("illegal instruction `{0:#X}` from address `{1:#X}`")]
    IllegalInstruction(u32, usize),
    #[error("unknown data error")]
    Unknown,
}
