// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use cid::Cid;
use fvm_ipld_encoding::tuple::*;
use fvm_ipld_encoding::{Cbor, RawBytes};
use fvm_shared::address::Address;

/// Init actor Constructor parameters
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ConstructorParams {
    pub network_name: String,
}

/// Init actor Exec Params
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ExecParams {
    pub code_cid: Cid,
    pub constructor_params: RawBytes,
}

/// Init actor Exec Return value
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct ExecReturn {
    /// ID based address for created actor
    pub id_address: Address,
    /// Reorg safe address for actor
    pub robust_address: Address,
}

impl Cbor for ExecReturn {}

/// Init actor Install Params
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct InstallParams {
    pub code: RawBytes,
}

/// Init actor Install Return value
#[derive(Serialize_tuple, Deserialize_tuple)]
pub struct InstallReturn {
    pub code_cid: Cid,
    pub installed: bool,
}

impl Cbor for InstallReturn {}
