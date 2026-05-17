//! Simplified contract which when executed releases the funds to beneficiary

use crate::{Contract, ContractWrapper};
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, CustomMsg, Deps, DepsMut, Empty, Env, MessageInfo,
    MigrateInfo, Response, StdError,
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantiateMsg {
    pub beneficiary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrateMsg {
    // just use some other string so we see there are other types
    pub new_guy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // returns InstantiateMsg
    Beneficiary {},
}

const HACKATOM: Item<InstantiateMsg> = Item::new("hackatom");

fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    HACKATOM.save(deps.storage, &msg)?;
    Ok(Response::default())
}

fn execute(deps: DepsMut, env: Env, _info: MessageInfo, _msg: Empty) -> Result<Response, StdError> {
    let init = HACKATOM.load(deps.storage)?;
    let balance = deps.querier.query_balance(env.contract.address, "btc")?;

    let resp = Response::new().add_message(BankMsg::Send {
        to_address: init.beneficiary,
        amount: vec![balance],
    });

    Ok(resp)
}

fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, StdError> {
    match msg {
        QueryMsg::Beneficiary {} => {
            let res = HACKATOM.load(deps.storage)?;
            to_json_binary(&res)
        }
    }
}

fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: MigrateMsg,
    info: MigrateInfo,
) -> Result<Response, StdError> {
    HACKATOM.update::<_, StdError>(deps.storage, |mut state| {
        state.beneficiary = msg.new_guy;
        Ok(state)
    })?;
    let resp = Response::new().add_attribute("migrate", "successful");
    Ok(resp)
}

pub fn contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query).with_migrate(migrate);
    Box::new(contract)
}

#[allow(dead_code)]
pub fn custom_contract<C>() -> Box<dyn Contract<C>>
where
    C: Clone + Debug + PartialEq + JsonSchema + CustomMsg + DeserializeOwned + 'static,
{
    let contract =
        ContractWrapper::new_with_empty(execute, instantiate, query).with_migrate_empty(migrate);
    Box::new(contract)
}
