pub mod contract;
pub mod execution;
pub mod handle_acknowledgement;
pub mod handle_inbound;
pub mod msg;
pub mod query;
mod state;

use cosmwasm_std::{DepsMut, Env};
pub use serde::{Deserialize, Serialize};
#[cfg(test)]
mod tests;

pub fn print_debug_logs(deps: DepsMut, env: &Env, log_data: &str) {
    let info_string = format!(
        "{}|info|{:?}\" height=\"{}\"",
        env.block.time.to_string(),
        log_data,
        env.block.height
    );
    deps.api.debug(&info_string);
}
