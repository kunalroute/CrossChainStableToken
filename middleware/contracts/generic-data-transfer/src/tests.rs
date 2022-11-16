use crate::contract::instantiate;
use crate::contract::{execute, fetch_white_listed_contract, is_white_listed_contract, sudo};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    DepsMut,
};
use cosmwasm_std::{Binary, CosmosMsg, Env, MessageInfo};
use ethabi::{encode, Token};
use router_wasm_bindings::ethabi;
use router_wasm_bindings::types::OutboundBatchRequest;
use router_wasm_bindings::{RouterMsg, SudoMsg};

const INIT_ADDRESS: &str = "init_address";
const TO: &str = "ab8483f64d9c6d1ecf9b849ae677dd3315835cb2";
const SENDER: &str = "00967d76a67fde3a1987b971a91cf4fc6db14a3d";
const CHAIN_ID: &str = "121";
const CHAIN_TYPE: u32 = 0;

fn do_instantiate(mut deps: DepsMut, env: Env, info: MessageInfo) {
    let instantiate_msg = InstantiateMsg {};
    let res = instantiate(deps.branch(), env.clone(), info.clone(), instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());
}

fn test_update_bridge_contract(mut deps: DepsMut, env: Env, info: MessageInfo) {
    let to_address: Vec<u8> = hex::decode(TO).unwrap();
    let msg: ExecuteMsg = ExecuteMsg::WhiteListApplicationContract {
        chain_id: String::from(CHAIN_ID),
        chain_type: CHAIN_TYPE,
        contract_address: to_address,
    };
    let result = execute(deps.branch(), env.clone(), info.clone(), msg);
    assert_eq!(result.is_ok(), true);

    let sender_address: Vec<u8> = hex::decode(SENDER).unwrap();
    let msg: ExecuteMsg = ExecuteMsg::WhiteListApplicationContract {
        chain_id: String::from(CHAIN_ID),
        chain_type: CHAIN_TYPE,
        contract_address: sender_address,
    };
    let result = execute(deps.branch(), env, info, msg);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_init() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(INIT_ADDRESS, &[]);
    do_instantiate(deps.as_mut(), env, info);
}

#[test]
fn test_sudo_function() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(INIT_ADDRESS, &[]);
    do_instantiate(deps.as_mut(), env.clone(), info.clone());
    test_update_bridge_contract(deps.as_mut(), env.clone(), info.clone());

    let chain_id_token: Token = Token::String(String::from(CHAIN_ID));

    let address_h160 = ethabi::ethereum_types::H160::from_slice(&hex::decode(TO).unwrap());
    let destination_address: Token = Token::Address(address_h160);

    let u256 = ethabi::ethereum_types::U256::from(CHAIN_TYPE);
    let chain_type = Token::Uint(u256);

    let instruction_bytes: Vec<u8> =
        hex::decode("00033132310000000000000000000000000000000000000000000000000000000000")
            .unwrap();
    let instruction: Token = Token::Bytes(instruction_bytes);

    let encoded: Vec<u8> = encode(&[destination_address, chain_id_token, chain_type, instruction]);
    let test_string: String = hex::encode(encoded);
    let encoded_string: String = base64::encode(test_string.clone());
    let msg: SudoMsg = SudoMsg::HandleInboundReq {
        sender: String::from("0x00967d76a67fde3a1987b971a91cf4fc6db14a3d"),
        chain_type: CHAIN_TYPE,
        source_chain_id: String::from(CHAIN_ID),
        payload: Binary::from_base64(&encoded_string).unwrap(),
    };

    let result = sudo(deps.as_mut(), env, msg);
    if result.is_err() {
        println!("{:?}", result.as_ref().err());
        assert!(false);
        return;
    }
    let response = result.unwrap();
    assert_eq!(response.messages.len(), 1);

    let message = response.messages.get(0).unwrap();
    let router_msg = message.msg.clone();
    match router_msg {
        CosmosMsg::Custom(msg) => match msg {
            RouterMsg::OutboundBatchRequests {
                outbound_batch_requests,
            } => {
                assert_eq!(outbound_batch_requests.len(), 1);
                let request: OutboundBatchRequest = outbound_batch_requests[0].clone();
                let contract_vec: Vec<u8> = request.contract_calls[0]
                    .destination_contract_address
                    .clone();
                let contract_addess: String = hex::encode(contract_vec);
                assert_eq!(request.destination_chain_id, String::from("121"));
                assert_eq!(request.destination_chain_type, 0);
                assert_eq!(contract_addess, TO);
            }
        },
        _ => {}
    }
}

#[test]
fn test_execute_update_bridge_address() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(INIT_ADDRESS, &[]);

    do_instantiate(deps.as_mut(), env.clone(), info.clone());
    test_update_bridge_contract(deps.as_mut(), env, info);

    let to_address: Vec<u8> = hex::decode(TO).unwrap();

    let data: bool = is_white_listed_contract(
        deps.as_ref(),
        String::from(CHAIN_ID),
        CHAIN_TYPE,
        to_address,
    )
    .unwrap();
    assert_eq!(data, true);
}

#[test]
fn test_list_white_listed_contracts() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(INIT_ADDRESS, &[]);

    do_instantiate(deps.as_mut(), env.clone(), info.clone());
    test_update_bridge_contract(deps.as_mut(), env, info);

    let data = fetch_white_listed_contract(deps.as_ref()).unwrap();

    println!("white_listed contracts {:?}", data);

    let sender_address: Vec<u8> = hex::decode(SENDER).unwrap();
    let (contract_2, id_2, type_2) = data[0].clone();
    assert_eq!(contract_2, sender_address);
    assert_eq!(id_2, CHAIN_ID);
    assert_eq!(type_2, CHAIN_TYPE);

    let to_address: Vec<u8> = hex::decode(TO).unwrap();
    let (contract_one, id_one, type_one) = data[1].clone();
    assert_eq!(to_address, contract_one);
    assert_eq!(id_one, CHAIN_ID);
    assert_eq!(type_one, CHAIN_TYPE);
}
