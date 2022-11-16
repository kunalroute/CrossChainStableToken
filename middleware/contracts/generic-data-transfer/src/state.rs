use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use router_wasm_bindings::Bytes;

pub const WHITE_LISTED_CONTRACTS: Map<(Vec<u8>, &str, u32), bool> =
    Map::new("white_listed_contracts");

pub const ACK_STATUS: Map<&str, Bytes> = Map::new("acknowledgement_status");

pub const OWNER: Item<Addr> = Item::new("owner");
