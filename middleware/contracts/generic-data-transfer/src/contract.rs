use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{ACK_STATUS, OWNER, WHITE_LISTED_CONTRACTS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{to_binary, Coin, Event, Order, StdError, SubMsg, Uint128};
use cw2::{get_contract_version, set_contract_version};
use ethabi::{decode, ParamType};
use router_wasm_bindings::ethabi;
use router_wasm_bindings::types::{ContractCall, OutboundBatchRequest};
use router_wasm_bindings::{RouterMsg, SudoMsg};

// version info for migration info
const CONTRACT_NAME: &str = "generic-data-transfer";
const CONTRACT_VERSION: &str = "0.1.1";

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
        } => update_bridge_contract(deps, env, info, chain_id, chain_type, contract_address),
        ExecuteMsg::DelistApplicationContract {
            chain_id,
            chain_type,
            contract_address,
        } => delist_application_contract(deps, env, info, chain_id, chain_type, contract_address),
    }
}

fn handle_in_bound_request(
    mut deps: DepsMut,
    env: Env,
    sender: String,
    src_chain_type: u32,
    src_chain_id: String,
    payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    let payload_bytes: Vec<u8> = base64::decode(payload.to_string()).unwrap();
    let mut hex_string: String = String::from_utf8(payload_bytes.clone()).unwrap();
    hex_string = hex_string.replace("0x", "");
    let hex_bytes: Vec<u8> = hex::decode(hex_string).unwrap();

    let decoded = match decode(
        &[
            ParamType::Address,
            ParamType::String,
            ParamType::Uint(64),
            ParamType::Bytes,
        ],
        &hex_bytes,
    ) {
        Ok(data) => data,
        Err(_) => {
            return Err(StdError::GenericErr {
                msg: String::from("Payload decoding error"),
            })
        }
    };
    let dest_contract_address: Vec<u8> = decoded[0].clone().into_address().unwrap().0.to_vec();
    let dest_chain_id: String = decoded[1].clone().into_string().unwrap();
    let dest_chain_type: u32 = decoded[2].clone().into_uint().unwrap().as_u32();
    let instruction: Vec<u8> = decoded[3].clone().into_bytes().unwrap();

    let info_str: String = format!(
        "handle_in_bound_request-- src_chain_type: {}, src_chain_id: {}, dest_chain_type: {}, dest_chain_type: {}",
        src_chain_type, src_chain_id.clone(), dest_chain_type, dest_chain_type
    );
    print_debug_logs(deps.branch(), &env, &info_str);

    let event = Event::new("in_bound_request")
        .add_attribute("sender", sender.to_string())
        .add_attribute("src_chain_type", src_chain_type.to_string())
        .add_attribute("src_chain_id", src_chain_id.clone());

    let sender_contract_address: Vec<u8> = match src_chain_type {
        0 => {
            let temp_sender = sender.replace("0x", "");
            hex::decode(temp_sender).unwrap()
        }
        _ => sender.as_bytes().to_vec(),
    };
    if !is_white_listed_contract(
        deps.as_ref(),
        dest_chain_id.clone(),
        dest_chain_type,
        dest_contract_address.clone(),
    )
    .unwrap()
    {
        return Err(StdError::GenericErr {
            msg: String::from("destination contract is not whitelisted"),
        });
    }
    if !is_white_listed_contract(
        deps.as_ref(),
        src_chain_id.clone(),
        src_chain_type,
        sender_contract_address,
    )
    .unwrap()
    {
        return Err(StdError::GenericErr {
            msg: String::from("src contract is not whitelisted"),
        });
    }

    let contract_call: ContractCall = ContractCall {
        destination_contract_address: dest_contract_address,
        payload: instruction,
    };
    let outbound_batch_req: OutboundBatchRequest = OutboundBatchRequest {
        destination_chain_type: dest_chain_type,
        destination_chain_id: dest_chain_id,
        contract_calls: vec![contract_call],
        relayer_fee: Coin {
            denom: String::from("router"),
            amount: Uint128::new(1000_000u128),
        },
        outgoing_tx_fee: Coin {
            denom: String::from("router"),
            amount: Uint128::new(1000_000u128),
        },
        is_atomic: false,
        exp_timestamp: None,
    };
    let outbound_batch_reqs: RouterMsg = RouterMsg::OutboundBatchRequests {
        outbound_batch_requests: vec![outbound_batch_req],
    };

    let res = Response::new()
        .add_submessage(SubMsg::new(outbound_batch_reqs))
        .add_event(event)
        .add_attribute("sender", sender)
        .add_attribute("chain_type", dest_chain_type.to_string())
        .add_attribute("src_chain_id", src_chain_id);
    Ok(res)
}

fn handle_out_bound_ack_request(
    mut deps: DepsMut,
    env: Env,
    sender: String,
    destination_chain_type: u64,
    destination_chain_id: String,
    outbound_batch_nonce: u64,
    contract_ack_responses: Binary,
    _execution_code: u8,
    _execution_status: bool,
) -> StdResult<Response<RouterMsg>> {
    let mut ack_status_key: String = destination_chain_id.clone();
    ack_status_key.push_str(&destination_chain_type.to_string());
    ack_status_key.push_str(&outbound_batch_nonce.to_string());

    ACK_STATUS.save(deps.storage, &ack_status_key, &contract_ack_responses.0)?;
    let info_str: String = format!(
        "handle_out_bound_ack_request-- destination_chain_type: {}, destination_chain_id: {}, sender: {}, outbound_batch_nonce: {}",
        destination_chain_type, destination_chain_id, sender.clone(), outbound_batch_nonce
    );
    print_debug_logs(deps.branch(), &env, &info_str);

    let res = Response::new()
        .add_attribute("sender", sender)
        .add_attribute("destination_chain_type", destination_chain_type.to_string())
        .add_attribute("destination_chain_id", destination_chain_id)
        .add_attribute("outbound_batch_nonce", outbound_batch_nonce.to_string())
        .add_attribute("contract_ack_responses", contract_ack_responses.to_string());
    Ok(res)
}

fn update_bridge_contract(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    chain_id: String,
    chain_type: u32,
    contract_address: Vec<u8>,
) -> StdResult<Response<RouterMsg>> {
    match OWNER.load(deps.storage) {
        Ok(owner) => {
            if owner != info.sender {
                return Err(StdError::GenericErr {
                    msg: "Unauthorised User".to_string(),
                });
            }
        }
        Err(err) => return Err(err),
    }
    let info_str: String = format!(
        "update_destinstion_contract_address-- chain_id: {:?}, chain_type {:?}, contract_address {:?}, sender: {:?}",
        chain_id.clone(),
        chain_type,
        contract_address,
        info.sender.to_string()
    );
    print_debug_logs(deps.branch(), &env, &info_str);

    WHITE_LISTED_CONTRACTS.save(
        deps.storage,
        (contract_address.clone(), &chain_id, chain_type),
        &true,
    )?;
    let res = Response::new()
        .add_attribute("chain_id", chain_id)
        .add_attribute("chain_type", chain_type.to_string())
        .add_attribute("bridge_address", hex::encode(contract_address));
    Ok(res)
}

fn delist_application_contract(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    chain_id: String,
    chain_type: u32,
    contract_address: Vec<u8>,
) -> StdResult<Response<RouterMsg>> {
    match OWNER.load(deps.storage) {
        Ok(owner) => {
            if owner != info.sender {
                return Err(StdError::GenericErr {
                    msg: "Unauthorised User".to_string(),
                });
            }
        }
        Err(err) => return Err(err),
    }
    let info_str: String = format!(
        "update_destinstion_contract_address-- chain_id: {:?}, chain_type {:?}, contract_address {:?}, sender: {:?}",
        chain_id.clone(),
        chain_type,
        contract_address,
        info.sender.to_string()
    );
    print_debug_logs(deps.branch(), &env, &info_str);

    WHITE_LISTED_CONTRACTS.remove(
        deps.storage,
        (contract_address.clone(), &chain_id, chain_type),
    );
    let res = Response::new()
        .add_attribute("chain_id", chain_id)
        .add_attribute("chain_type", chain_type.to_string())
        .add_attribute("bridge_address", hex::encode(contract_address));
    Ok(res)
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
    }
}

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
fn print_debug_logs(deps: DepsMut, env: &Env, log_data: &str) {
    let info_string = format!(
        "{}|info|{:?}\" height=\"{}\"",
        env.block.time.to_string(),
        log_data,
        env.block.height
    );
    deps.api.debug(&info_string);
}
