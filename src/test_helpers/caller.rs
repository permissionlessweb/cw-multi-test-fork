use crate::{Contract, ContractWrapper};
use cosmwasm_std::{
    Binary, CustomMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult,
    SubMsg, WasmMsg,
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> Result<Response, StdError> {
    Ok(Response::default())
}

fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: WasmMsg,
) -> Result<Response, StdError> {
    let message = SubMsg::new(msg);

    Ok(Response::new().add_submessage(message))
}

fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary, StdError> {
    Err(StdError::msg(
        "query not implemented for the `caller` contract",
    ))
}

pub fn contract<C>() -> Box<dyn Contract<C>>
where
    C: Clone + Debug + PartialEq + JsonSchema + CustomMsg + DeserializeOwned + 'static,
{
    let contract = ContractWrapper::new_with_empty(execute, instantiate, query);
    Box::new(contract)
}
