use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use router_wasm_bindings::types::OutboundBatchRequest;

pub const WHITE_LISTED_CONTRACTS: Map<(Vec<u8>, &str, u32), bool> =
    Map::new("white_listed_contracts");

pub const OWNER: Item<Addr> = Item::new("owner");

pub const ACK_STATUS: Map<&str, String> = Map::new("acknowledgement_status");

pub const LAST_OUTBOUND_NONCE: Item<u64> = Item::new("last_outbound_nonce");
pub const TEMP_STATE_CREATE_OUTBOUND_REPLY_ID: Item<Vec<OutboundBatchRequest>> =
    Item::new("temp_state_create_outbound_reply_id");

pub const OUTBOUND_CALLS_STATE: Map<&str, OutboundBatchRequest> = Map::new("outbound_calls_state");
