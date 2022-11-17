use crate::execution::{
    delist_application_contract, update_white_listed_application_contracts,
    white_list_application_contract,
};
use crate::handle_acknowledgement::handle_out_bound_ack_request;
use crate::handle_inbound::handle_in_bound_request;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::print_debug_logs;
use crate::query::{
    fetch_ack_data, fetch_contract_calls, fetch_recent_out_bound_nonce, fetch_temp_state,
    fetch_white_listed_contract, is_white_listed_contract,
};
use crate::state::{OWNER, TEMP_STATE_CREATE_OUTBOUND_REPLY_ID, LAST_OUTBOUND_NONCE, OUTBOUND_CALLS_STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{to_binary, StdError, Reply, SubMsgResult, from_binary};
use cw2::{get_contract_version, set_contract_version};
use router_wasm_bindings::types::{OutboundBatchResponses, OutboundBatchResponse, OutboundBatchRequest};
use router_wasm_bindings::{RouterMsg, SudoMsg};

// version info for migration info
const CONTRACT_NAME: &str = "generic-data-transfer";
const CONTRACT_VERSION: &str = "0.1.1";
pub const REQUEST_TIMEOUT: u64 = 600;
pub const CREATE_OUTBOUND_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("Instantiating the contract🚀");
    print_debug_logs(
        deps.branch(),
        &env,
        &format!("the contract initiator {:?}", info.sender),
    );
    OWNER.save(deps.storage, &info.sender.clone())?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("action", "generic-data-transfer intstantiated"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> StdResult<Response<RouterMsg>> {
    match msg {
        SudoMsg::HandleInboundReq {
            sender,
            chain_type,
            source_chain_id,
            payload,
        } => handle_in_bound_request(deps, env, sender, chain_type, source_chain_id, payload),
        SudoMsg::HandleOutboundAck {
            outbound_tx_requested_by,
            destination_chain_type,
            destination_chain_id,
            outbound_batch_nonce,
            contract_ack_responses,
            execution_code,
            execution_status,
        } => handle_out_bound_ack_request(
            deps,
            env,
            outbound_tx_requested_by,
            destination_chain_type,
            destination_chain_id,
            outbound_batch_nonce,
            contract_ack_responses,
            execution_code,
            execution_status,
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<RouterMsg>> {
    match msg {
        ExecuteMsg::WhiteListApplicationContract {
            chain_id,
            chain_type,
            contract_address,
        } => {
            white_list_application_contract(deps, env, info, chain_id, chain_type, contract_address)
        }
        ExecuteMsg::WhiteListApplicationContracts { contracts } => {
            update_white_listed_application_contracts(deps, env, info, contracts)
        }
        ExecuteMsg::DelistApplicationContract {
            chain_id,
            chain_type,
            contract_address,
        } => delist_application_contract(deps, env, info, chain_id, chain_type, contract_address),
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        CREATE_OUTBOUND_REPLY_ID => {
            deps.api.debug(&msg.id.to_string());
            // TODO: need to handle nonce data here, logic depends on the msg binary data structure.
            let mut response: Response = Response::new();
            match msg.result {
                SubMsgResult::Ok(msg_result) => match msg_result.data {
                    Some(binary_data) => {
                        deps.api.debug("Binary Data Found");
                        deps.api.debug(&binary_data.to_string());
                        let outbound_responses: OutboundBatchResponses =
                            from_binary(&binary_data).unwrap();

                        let response_vec: Vec<OutboundBatchResponse> =
                            outbound_responses.outbound_batch_responses;
                        let temp_state_vec: Vec<OutboundBatchRequest> =
                            TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.load(deps.storage)?;
                        deps.api.debug("Handling the nonce info");
                        for i in 0..response_vec.len() {
                            let nonce: u64 = response_vec[i].outbound_batch_nonce;
                            LAST_OUTBOUND_NONCE.save(deps.storage, &nonce)?;
                            let temp_state: &OutboundBatchRequest = &temp_state_vec[i];
                            let mut ack_status_key: String =
                                temp_state.destination_chain_id.clone();
                            ack_status_key.push_str(&temp_state.destination_chain_type.to_string());
                            ack_status_key.push_str(&nonce.to_string());
                            deps.api.debug(&ack_status_key);
                            OUTBOUND_CALLS_STATE.save(deps.storage, &ack_status_key, temp_state)?;
                            let mut att_key = String::from("outbound_nonce_");
                            att_key.push_str(&i.to_string());
                            response = response.add_attribute(att_key, nonce.to_string());
                        }
                    }
                    None => deps.api.debug("No Binary Data Found"),
                },
                SubMsgResult::Err(err) => deps.api.debug(&err.to_string()),
            }
            return Ok(response);
        }
        id => return Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(mut deps: DepsMut, env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME.to_string() {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    if ver.version >= CONTRACT_VERSION.to_string() {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    let info_str: String = format!(
        "migrating contract: {}, new_contract_version: {}, contract_name: {}",
        env.contract.address,
        CONTRACT_VERSION.to_string(),
        CONTRACT_NAME.to_string()
    );
    print_debug_logs(deps.branch(), &env, &info_str);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        QueryMsg::IsWhiteListedContract {
            chain_id,
            chain_type,
            contract_address,
        } => to_binary(&is_white_listed_contract(
            deps,
            chain_id,
            chain_type,
            contract_address,
        )?),
        QueryMsg::FetchWhiteListedContract {} => to_binary(&fetch_white_listed_contract(deps)?),
        QueryMsg::FetchAckData {
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        } => to_binary(&fetch_ack_data(
            deps,
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        )?),
        QueryMsg::FetchContractCalls {
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        } => to_binary(&fetch_contract_calls(
            deps,
            destination_chain_id,
            destination_chain_type,
            outbound_batch_nonce,
        )?),
        QueryMsg::FetchTempItem {} => to_binary(&fetch_temp_state(deps)?),
        QueryMsg::FetchRecentOutboundNonce {} => to_binary(&fetch_recent_out_bound_nonce(deps)?),
    }
}
