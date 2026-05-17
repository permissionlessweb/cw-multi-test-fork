pub mod bank;
pub mod mock_querier;
pub mod staking;
pub mod wasm;
use cosmwasm_std::{StdResult, Storage};

pub use mock_querier::MockQuerier;
pub mod gas;

use crate::wasm_emulation::channel::RemoteChannel;

use super::input::{BankStorage, WasmStorage};

pub trait ContainsRemote {
    fn with_remote(self, remote: RemoteChannel) -> Self;

    fn set_remote(&mut self, remote: RemoteChannel);
}

pub trait AllWasmQuerier {
    fn query_all(&self, storage: &dyn Storage) -> StdResult<WasmStorage>;
}

pub trait AllBankQuerier {
    fn query_all(&self, storage: &dyn Storage) -> StdResult<BankStorage>;
}
