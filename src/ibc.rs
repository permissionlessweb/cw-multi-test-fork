//! Ibc Module adds IBC support to cw-multi-test
#![allow(missing_docs)]
use cosmwasm_std::{
    Empty, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcMsg, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcQuery,
};

use crate::{AcceptingModule, FailingModule, Module};

pub trait Ibc: Module<ExecT = IbcMsg, QueryT = IbcQuery, SudoT = Empty> {}

pub type IbcAcceptingModule = AcceptingModule<IbcMsg, IbcQuery, Empty>;

impl Ibc for IbcAcceptingModule {}

pub type IbcFailingModule = FailingModule<IbcMsg, IbcQuery, Empty>;

impl Ibc for IbcFailingModule {}
