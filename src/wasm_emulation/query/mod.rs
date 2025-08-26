pub mod bank;
pub mod mock_querier;
pub mod staking;
pub mod wasm;
use cosmwasm_std::{StdResult, Storage};

pub use mock_querier::MockQuerier;
pub mod gas;

use super::input::{BankStorage, WasmStorage};

pub trait AllWasmQuerier {
    fn query_all(&self, storage: &dyn Storage) -> StdResult<WasmStorage>;
}

pub trait AllBankQuerier {
    fn query_all(&self, storage: &dyn Storage) -> StdResult<BankStorage>;
}
