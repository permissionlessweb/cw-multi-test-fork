#![cfg(test)]

use cosmwasm_std::Empty;
use cw_orch::daemon::{networks::XION_TESTNET_1, RUNTIME};
use cw_orch::prelude::ChainInfo;

use crate::{
    no_init,
    wasm_emulation::{channel::RemoteChannel, query::ContainsRemote},
    App, AppBuilder, WasmKeeper,
};

pub const CHAIN: ChainInfo = XION_TESTNET_1;
pub fn remote_channel() -> RemoteChannel {
    RemoteChannel::new(
        &RUNTIME,
        CHAIN.grpc_urls,
        CHAIN.chain_id,
        CHAIN.network_info.pub_address_prefix,
    )
    .unwrap()
}

pub fn default_app() -> App {
    let remote_channel = remote_channel();
    let wasm = WasmKeeper::<Empty, Empty>::new().with_remote(remote_channel.clone());
    AppBuilder::default()
        .with_wasm(wasm)
        .with_remote(remote_channel.clone())
        .build(no_init)
}

mod test_app;
mod test_custom_handler;
mod test_gov;
mod test_ibc;
