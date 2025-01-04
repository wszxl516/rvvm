use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("store address fault from `{0:#X}`")]
    StoreAddressFault(u64),
    #[error("load  address fault from `{0:#X}`")]
    LoadAddressFault(u64),
    #[error("address `{0:#X}` out of range")]
    AddressOutOfRange(u64),
    #[error("illegal instruction `{0:#X}` from address `{1:#X}`")]
    IllegalInstruction(u32, u64),
    #[error("unknown data error")]
    Unknown,
}