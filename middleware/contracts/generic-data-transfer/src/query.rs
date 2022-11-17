use cosmwasm_std::{Deps, Order, StdResult};
use router_wasm_bindings::types::OutboundBatchRequest;

use crate::state::{
    ACK_STATUS, LAST_OUTBOUND_NONCE, OUTBOUND_CALLS_STATE, TEMP_STATE_CREATE_OUTBOUND_REPLY_ID,
    WHITE_LISTED_CONTRACTS,
};

pub fn is_white_listed_contract(
    deps: Deps,
    chain_id: String,
    chain_type: u32,
    contract_address: Vec<u8>,
) -> StdResult<bool> {
    match WHITE_LISTED_CONTRACTS.load(
        deps.storage,
        (contract_address.clone(), &chain_id, chain_type),
    ) {
        Ok(data) => Ok(data),
        Err(_) => Ok(false),
    }
}

pub fn fetch_white_listed_contract(deps: Deps) -> StdResult<Vec<(Vec<u8>, String, u32)>> {
    let keys: Vec<(Vec<u8>, String, u32)> = match WHITE_LISTED_CONTRACTS
        .keys(deps.storage, None, None, Order::Ascending)
        .collect()
    {
        Ok(data) => data,
        Err(err) => return Err(err),
    };
    return Ok(keys);
}

pub fn fetch_ack_data(
    deps: Deps,
    destination_chain_id: String,
    destination_chain_type: u64,
    outbound_batch_nonce: u64,
) -> StdResult<String> {
    let mut ack_status_key: String = destination_chain_id.clone();
    ack_status_key.push_str(&destination_chain_type.to_string());
    ack_status_key.push_str(&outbound_batch_nonce.to_string());

    ACK_STATUS.load(deps.storage, &ack_status_key)
}

pub fn fetch_contract_calls(
    deps: Deps,
    destination_chain_id: String,
    destination_chain_type: u64,
    outbound_batch_nonce: u64,
) -> StdResult<OutboundBatchRequest> {
    let mut ack_status_key: String = destination_chain_id.clone();
    ack_status_key.push_str(&destination_chain_type.to_string());
    ack_status_key.push_str(&outbound_batch_nonce.to_string());

    OUTBOUND_CALLS_STATE.load(deps.storage, &ack_status_key)
}

pub fn fetch_temp_state(deps: Deps) -> StdResult<Vec<OutboundBatchRequest>> {
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.load(deps.storage)
}

pub fn fetch_recent_out_bound_nonce(deps: Deps) -> StdResult<u64> {
    LAST_OUTBOUND_NONCE.load(deps.storage)
}
