use near_sdk::{Gas, Balance};

pub const GAS_FOR_FT_TRANSFER: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_CALLBACK_METHOD: Gas = Gas(30_000_000_000_000);
pub const DEPOSIT_ONE_YOCTO: Balance = 1;