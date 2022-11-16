use crate::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // here user can define other executable messages
    WhiteListApplicationContract {
        chain_id: String,
        chain_type: u32,
        contract_address: Vec<u8>,
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
}
