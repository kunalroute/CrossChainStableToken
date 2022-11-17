use crate::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    WhiteListApplicationContract {
        chain_id: String,
        chain_type: u32,
        contract_address: Vec<u8>,
    },
    WhiteListApplicationContracts {
        contracts: Vec<ApplicationContract>,
    },
    DelistApplicationContract {
        chain_id: String,
        chain_type: u32,
        contract_address: Vec<u8>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // fetch contract version
    GetContractVersion {},
    // will check contract's whitelisted status
    IsWhiteListedContract {
        chain_id: String,
        chain_type: u32,
        contract_address: Vec<u8>,
    },
    FetchWhiteListedContract {},
    FetchAckData {
        destination_chain_id: String,
        destination_chain_type: u64,
        outbound_batch_nonce: u64,
    },
    FetchContractCalls {
        destination_chain_id: String,
        destination_chain_type: u64,
        outbound_batch_nonce: u64,
    },
    FetchTempItem {},
    FetchRecentOutboundNonce {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ApplicationContract {
    pub chain_id: String,
    pub chain_type: u32,
    pub contract_address: Vec<u8>,
}
