//! # Error definitions
pub use anyhow::{anyhow, Context as AnyContext, Error as AnyError, Result as AnyResult};

use cosmwasm_std::{WasmMsg, WasmQuery};
use thiserror::Error;

macro_rules! std_error_bail {
    ($msg:literal $(,)?) => {
        return Err(cosmwasm_std::StdError::msg($msg))
    };
    ($err:expr $(,)?) => {
        return Err(cosmwasm_std::StdError::msg($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(cosmwasm_std::StdError::msg(format!($fmt, $($arg)*)))
    };
}

pub(crate) use std_error_bail;

macro_rules! std_error {
    ($msg:literal $(,)?) => {
        cosmwasm_std::StdError::msg($msg)
    };
    ($err:expr $(,)?) => {
        cosmwasm_std::StdError::msg($err)
    };
    ($fmt:expr, $($arg:tt)*) => {
        cosmwasm_std::StdError::msg(format!($fmt, $($arg)*))
    };
}

pub(crate) use std_error;
/// An enumeration of errors reported across the **CosmWasm MultiTest** library.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    /// Error variant for reporting an empty attribute key.
    #[error("Empty attribute key. Value: {0}")]
    EmptyAttributeKey(String),

    /// Error variant for reporting an empty attribute value.
    #[deprecated(note = "This error is not reported anymore. Will be removed in next release.")]
    #[error("Empty attribute value. Key: {0}")]
    EmptyAttributeValue(String),

    /// Error variant for reporting a usage of reserved key prefix.
    #[error("Attribute key starts with reserved prefix _: {0}")]
    ReservedAttributeKey(String),

    /// Error variant for reporting too short event types.
    #[error("Event type too short: {0}")]
    EventTypeTooShort(String),

    /// Error variant for reporting that unsupported wasm query was encountered during processing.
    #[error("Unsupported wasm query: {0:?}")]
    UnsupportedWasmQuery(WasmQuery),

    /// Error variant for reporting that unsupported wasm message was encountered during processing.
    #[error("Unsupported wasm message: {0:?}")]
    UnsupportedWasmMsg(WasmMsg),

    /// Error variant for reporting invalid contract code.
    #[error("code id: invalid")]
    InvalidCodeId,

    /// Error variant for reporting unregistered contract code.
    #[error("code id {0}: no such code")]
    UnregisteredCodeId(u64),

    /// Error variant for reporting duplicated contract code identifier.
    #[error("duplicated code id {0}")]
    DuplicatedCodeId(u64),

    /// Error variant for reporting a situation when no more contract code identifiers are available.
    #[error("no more code identifiers available")]
    NoMoreCodeIdAvailable,

    /// Error variant for reporting duplicated contract addresses.
    #[error("Contract with this address already exists: {0}")]
    DuplicatedContractAddress(String),
}

/// Creates an instance of the error for empty attribute key.
pub fn empty_attribute_key(value: impl Into<String>) -> String {
    format!("Empty attribute key. Value: {0}", value.into())
}

/// Creates an instance of the error when reserved attribute key was used.
pub fn reserved_attribute_key(key: impl Into<String>) -> String {
    format!(
        "Attribute key starts with reserved prefix _: {0}",
        key.into()
    )
}

/// Creates an instance of the error for too short event types.
pub fn event_type_too_short(ty: impl Into<String>) -> String {
    format!("Event type too short: {0}", ty.into())
}

/// Creates an instance of the error for unsupported wasm queries.
pub fn unsupported_wasm_query(query: WasmQuery) -> String {
    format!("Unsupported wasm query: {query:?}")
}

/// Creates an instance of the error for unsupported wasm messages.
pub fn unsupported_wasm_message(msg: WasmMsg) -> String {
    format!("Unsupported wasm message: {msg:?}")
}

/// Creates an instance of the error for invalid contract code identifier.
pub fn invalid_code_id() -> String {
    "code id: invalid".to_string()
}

/// Creates an instance of the error for unregistered contract code identifier.
pub fn unregistered_code_id(code_id: u64) -> String {
    format!("code id {code_id}: no such code")
}

/// Creates an instance of the error for duplicated contract code identifier.
pub fn duplicated_code_id(code_id: u64) -> String {
    format!("duplicated code id {code_id}")
}

/// Creates an instance of the error for exhausted contract code identifiers.
pub fn no_more_code_id_available() -> String {
    "no more code identifiers available".to_string()
}

/// Creates an instance of the error for duplicated contract addresses.
pub fn duplicated_contract_address(addr: impl Into<String>) -> String {
    format!(
        "Contract with this address already exists: {0}",
        addr.into()
    )
}
