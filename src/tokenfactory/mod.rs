use anyhow::{bail, Result as AnyResult};
use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{
    to_json_binary, Addr, AnyMsg, Api, Binary, BlockInfo, CosmosMsg, CustomMsg, CustomQuery, Empty,
    GrpcQuery, Querier, Storage,
};
use cw_storage_plus::Map;
use prost::Message;

use crate::{AppResponse, CosmosRouter, Stargate};

pub mod osmosis;
pub(crate) mod serde;
pub use shim::Coin;

pub(crate) mod shim;

/// Always accepting handler for `Stargate`/`Any` message variants and `Stargate`/`Grpc` queries.
pub struct TokenFactoryStargate;

const TOKENFACTORY_DENOMS: Map<(&String, &String), ()> = Map::new("tokenfactory_denoms");

/// Merge sender and subdenom to create the denom
pub fn denom(sender: &str, subdenom: &str) -> String {
    format!("factory/{sender}/{subdenom}")
}

fn split_denom(denom: &str) -> AnyResult<(String, String)> {
    let split_result: Vec<&str> = denom.splitn(3, "/").collect();

    if split_result.len() != 3 {
        bail!("Error decoding denom {denom} into factory/sender/subdenom")
    }
    if split_result[0] != "factory" {
        bail!("Error decoding denom {denom} into factory/sender/subdenom")
    }

    Ok((split_result[1].to_string(), split_result[2].to_string()))
}

impl Stargate for TokenFactoryStargate {
    fn execute_stargate<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        type_url: String,
        value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        // We match the type url
        match type_url.as_str() {
            osmosis::MsgCreateDenom::TYPE_URL => {
                let msg = osmosis::MsgCreateDenom::decode(value.as_slice())?;
                if sender != api.addr_validate(&msg.sender)? {
                    bail!("Sender field should match the message sender")
                }
                self.create_denom(storage, msg.sender, msg.subdenom)
            }
            osmosis::MsgMint::TYPE_URL => {
                let msg = osmosis::MsgMint::decode(value.as_slice())?;
                if sender != api.addr_validate(&msg.sender)? {
                    bail!("Sender field should match the message sender")
                }
                self.mint(
                    api,
                    storage,
                    router,
                    block,
                    msg.sender,
                    msg.amount,
                    msg.mint_to_address,
                )
            }
            osmosis::MsgBurn::TYPE_URL => {
                let msg = osmosis::MsgBurn::decode(value.as_slice())?;
                if sender != api.addr_validate(&msg.sender)? {
                    bail!("Sender field should match the message sender")
                }
                self.burn(
                    api,
                    storage,
                    router,
                    block,
                    msg.sender,
                    msg.amount,
                    msg.burn_from_address,
                )
            }
            _ => {
                bail!("type url {} is not supported", type_url)
            }
        }
    }

    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _path: String,
        _data: Binary,
    ) -> AnyResult<Binary> {
        bail!("Stargate / any queries, unsupported")
    }

    fn execute_any<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: Addr,
        msg: AnyMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        self.execute_stargate(api, storage, router, block, sender, msg.type_url, msg.value)
    }

    fn query_grpc(
        &self,
        api: &dyn Api,
        storage: &dyn Storage,
        querier: &dyn Querier,
        block: &BlockInfo,
        request: GrpcQuery,
    ) -> AnyResult<Binary> {
        self.query_stargate(api, storage, querier, block, request.path, request.data)
    }
}

impl TokenFactoryStargate {
    fn create_denom(
        &self,
        storage: &mut dyn Storage,
        sender: String,
        subdenom: String,
    ) -> AnyResult<AppResponse> {
        if TOKENFACTORY_DENOMS.has(storage, (&sender, &subdenom)) {
            bail!("Subdenom {subdenom} by sender {sender} already exists")
        }

        TOKENFACTORY_DENOMS.save(storage, (&sender, &subdenom), &())?;

        let data = osmosis::MsgCreateDenomResponse {
            new_token_denom: denom(&sender, &subdenom),
        }
        .to_proto_bytes();

        Ok(AppResponse {
            data: Some(data.into()),
            events: vec![],
        })
    }
    fn mint<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: String,
        amount: Option<Coin>,
        mint_to_address: String,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        let amount = if let Some(amount) = amount {
            amount
        } else {
            bail!("Can't mint nothing, please specify an amount")
        };

        // We verify the denom exists
        let (creator, subdenom) = split_denom(&amount.denom)?;
        if creator != sender {
            bail!("Only creator can mint token factory tokens")
        }
        if !TOKENFACTORY_DENOMS.has(storage, (&sender, &subdenom)) {
            bail!("Subdenom {subdenom} by sender {sender} doesn't exist")
        }

        // We mint. No need for transactional cache here, if this fails, the tokenfactory mint fails
        router.sudo(
            api,
            storage,
            block,
            crate::SudoMsg::Bank(crate::BankSudo::Mint {
                to_address: mint_to_address,
                amount: vec![amount.try_into()?],
            }),
        )?;

        let data = osmosis::MsgMintResponse {}.to_proto_bytes();

        Ok(AppResponse {
            data: Some(data.into()),
            events: vec![],
        })
    }

    fn burn<ExecC, QueryC>(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &BlockInfo,
        sender: String,
        amount: Option<Coin>,
        burn_from_address: String,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        let amount = if let Some(amount) = amount {
            amount
        } else {
            bail!("Can't burn nothing, please specify an amount")
        };
        let (creator, subdenom) = split_denom(&amount.denom)?;
        if creator != sender {
            bail!("Only creator can burn token factory tokens from any address")
        }
        // We verify the denom exists
        if !TOKENFACTORY_DENOMS.has(storage, (&sender, &subdenom)) {
            bail!("Subdenom {subdenom} by sender {sender} doesn't exist")
        }

        // We burn. No need for transactional cache here, if this fails, the tokenfactory burn fails
        router.execute(
            api,
            storage,
            block,
            api.addr_validate(&burn_from_address)?,
            CosmosMsg::Bank(cosmwasm_std::BankMsg::Burn {
                amount: vec![amount.try_into()?],
            }),
        )?;
        let data = osmosis::MsgBurnResponse {}.to_proto_bytes();

        Ok(AppResponse {
            data: Some(data.into()),
            events: vec![],
        })
    }
}
