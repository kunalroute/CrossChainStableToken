use crate::{print_debug_logs, state::ACK_STATUS};
use cosmwasm_std::{DepsMut, Env, Response, StdResult};
use router_wasm_bindings::RouterMsg;

pub fn handle_out_bound_ack_request(
    mut deps: DepsMut,
    env: Env,
    sender: String,
    destination_chain_type: u32,
    destination_chain_id: String,
    outbound_batch_nonce: u64,
    contract_ack_responses: Vec<bool>,
    execution_code: u64,
    execution_status: bool,
) -> StdResult<Response<RouterMsg>> {
    let dest_chain_type: u32 = destination_chain_type;
    let mut ack_status_key: String = destination_chain_id.clone();
    ack_status_key.push_str(&dest_chain_type.to_string());
    ack_status_key.push_str(&outbound_batch_nonce.to_string());

    for i in 0..contract_ack_responses.len() {
        let msg: String = format!(
            "contract_addr --, ack_status {:?}",
            // contract_ack_responses[i].destination_contract_address,
            contract_ack_responses[i]
        );
        deps.api.debug(&msg);
    }
    let execution_msg: String = format!(
        "execution_code {:?}, execution_status {:?}, contract_ack_responses {:?}",
        execution_code, execution_status, contract_ack_responses
    );
    deps.api.debug(&execution_msg);
    ACK_STATUS.save(deps.storage, &ack_status_key, &execution_msg)?;

    let info_str: String = format!(
        "handle_out_bound_ack_request-- destination_chain_type: {}, destination_chain_id: {}, sender: {}, outbound_batch_nonce: {}",
        dest_chain_type, destination_chain_id, sender.clone(), outbound_batch_nonce
    );
    print_debug_logs(deps.branch(), &env, &info_str);

    let res = Response::new()
        .add_attribute("sender", sender)
        .add_attribute("destination_chain_type", dest_chain_type.to_string())
        .add_attribute("destination_chain_id", destination_chain_id)
        .add_attribute("outbound_batch_nonce", outbound_batch_nonce.to_string());
    Ok(res)
}
