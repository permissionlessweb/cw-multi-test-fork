use cosmwasm_std::{Addr, Coin, StdResult};

use crate::wasm_emulation::channel::RemoteChannel;

pub struct BankRemoteQuerier;

impl BankRemoteQuerier {
    pub fn get_balance(remote: RemoteChannel, account: &Addr) -> StdResult<Vec<Coin>> {
        let querier = cw_orch::daemon::queriers::Bank {
            channel: remote.channel,
            rt_handle: Some(remote.rt.clone()),
        };
        let distant_amounts: Vec<Coin> = remote
            .rt
            .block_on(querier._spendable_balances(account))
            .unwrap();
        Ok(distant_amounts)
    }
}
