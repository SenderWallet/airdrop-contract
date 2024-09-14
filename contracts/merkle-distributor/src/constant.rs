use near_gas::NearGas;

pub const GAS_FOR_FT_TRANSFER: NearGas = NearGas::from_tgas(10);
pub const GAS_FOR_VIEW_METHOD: NearGas = NearGas::from_tgas(5);
pub const GAS_FOR_CALLBACK_METHOD: NearGas = NearGas::from_tgas(30);
