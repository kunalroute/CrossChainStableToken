use crate::contract::CREATE_OUTBOUND_REPLY_ID;
use crate::print_debug_logs;
//use crate::query::is_white_listed_contract;
use crate::state::TEMP_STATE_CREATE_OUTBOUND_REPLY_ID;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{Binary, DepsMut, Env, Response, StdResult};
use cosmwasm_std::{Coin, Event, ReplyOn, StdError, SubMsg, Uint128};
use router_wasm_bindings::ethabi::{decode, ParamType};
use router_wasm_bindings::types::{ContractCall, OutboundBatchRequest};
use router_wasm_bindings::RouterMsg;

pub const REQUEST_TIMEOUT: u64 = 24 * 60 * 60;

pub fn handle_in_bound_request(
    mut deps: DepsMut,
    env: Env,
    sender: String,
    src_chain_type: u32,
    src_chain_id: String,
    payload: Binary,
) -> StdResult<Response<RouterMsg>> {
    let hex_bytes: Vec<u8> = base64::decode(payload.to_string()).unwrap();

    let dest_chain_id: String;
    let dest_contract_address: [u8; 20];
    let dest_chain_type: u32;
    let instruction: Vec<u8> = hex_bytes.clone();
    
    {
        let method_type: u8;
        let inner_payload: Vec<u8>;

        let decoded_payload = match decode(
            &[
                ParamType::Uint(8),
                ParamType::Bytes
            ],
            &hex_bytes,
        ) {
            Ok(tokens) => tokens,
            Err(_) => {
                return Err(StdError::GenericErr {
                    msg: String::from("Payload decoding error - main payload"),
                })
            }
        };
        method_type = decoded_payload[0].clone().into_uint().unwrap().as_u32() as u8;
        inner_payload = decoded_payload[1].clone().into_bytes().unwrap();
        dest_chain_type = 0;

        if method_type == 0 {
            let decoded_inner_payload = match decode(
                &[
                    ParamType::Uint(8),
                    ParamType::Uint(256),
                    ParamType::Uint(256),
                    ParamType::Address,
                    ParamType::Address,
                ],
                &inner_payload,
            ) {
                Ok(tokens) => tokens,
                Err(_) => {
                    return Err(StdError::GenericErr {
                        msg: String::from("Payload decoding error"),
                    })
                }
            };
            dest_chain_id = decoded_inner_payload[0].clone().into_uint().unwrap().to_string();
            dest_contract_address = decoded_inner_payload[4].clone().into_address().unwrap().0;
        } else if method_type == 1 {
            let decoded_inner_payload = match decode(
                &[
                    ParamType::Uint(8),
                    ParamType::Uint(256),
                    ParamType::Uint(256),
                    ParamType::Address
                ],
                &inner_payload,
            ) {
                Ok(tokens) => tokens,
                Err(_) => {
                    return Err(StdError::GenericErr {
                        msg: String::from("Payload decoding error"),
                    })
                }
            };
            dest_chain_id = decoded_inner_payload[0].clone().into_uint().unwrap().to_string();
            dest_contract_address = decoded_inner_payload[3].clone().into_address().unwrap().0;
        } else if method_type == 2 {
            let decoded_inner_payload = match decode(
                &[
                    ParamType::Uint(8),
                    ParamType::Address,
                    ParamType::Uint(256),
                    ParamType::Address
                ],
                &inner_payload,
            ) {
                Ok(tokens) => tokens,
                Err(_) => {
                    return Err(StdError::GenericErr {
                        msg: String::from("Payload decoding error"),
                    })
                }
            };
            dest_chain_id = decoded_inner_payload[0].clone().into_uint().unwrap().to_string();
            dest_contract_address = decoded_inner_payload[3].clone().into_address().unwrap().0;

        } else {
            return Err(StdError::GenericErr {
                msg: String::from("Payload decoding error - methodType invalid"),
            })
        }
        


    }

    let info_str: String = format!(
        "handle_in_bound_request-- src_chain_type: {}, src_chain_id: {}, destination_chain_type: {}, destination_chain_id: {}",
        src_chain_type, src_chain_id.clone(), dest_chain_type, dest_chain_id
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

    // TODO; handle whitelisting
    // if !is_white_listed_contract(
    //     deps.as_ref(),
    //     dest_chain_id.clone(),
    //     dest_chain_type,
    //     dest_contract_address.to_vec(),
    // )
    // .unwrap()
    // {
    //     return Err(StdError::GenericErr {
    //         msg: String::from("destination contract is not whitelisted"),
    //     });
    // }
    // if !is_white_listed_contract(
    //     deps.as_ref(),
    //     src_chain_id.clone(),
    //     src_chain_type,
    //     sender_contract_address,
    // )
    // .unwrap()
    // {
    //     return Err(StdError::GenericErr {
    //         msg: String::from("src contract is not whitelisted"),
    //     });
    // }

    let contract_call: ContractCall = ContractCall {
        destination_contract_address: dest_contract_address.to_vec(),
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
        exp_timestamp: env.block.time.seconds() + REQUEST_TIMEOUT,
    };

    let outbound_batch_requests: Vec<OutboundBatchRequest> = vec![outbound_batch_req];
    TEMP_STATE_CREATE_OUTBOUND_REPLY_ID.save(deps.storage, &outbound_batch_requests)?;
    let outbound_batch_reqs: RouterMsg = RouterMsg::OutboundBatchRequests {
        outbound_batch_requests,
    };

    let outbound_submessage = SubMsg {
        gas_limit: None,
        id: CREATE_OUTBOUND_REPLY_ID,
        reply_on: ReplyOn::Success,
        msg: outbound_batch_reqs.into(),
    };
    let res = Response::new()
        .add_submessage(outbound_submessage)
        .add_event(event)
        .add_attribute("sender", sender)
        .add_attribute("chain_type", src_chain_type.to_string())
        .add_attribute("src_chain_id", src_chain_id);
    Ok(res)
}
