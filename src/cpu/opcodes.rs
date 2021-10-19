use crate::cpu::CPU;

pub const NOP: fn() = || {};

pub type OperationDetails = (&'static str, (u8, u8));
