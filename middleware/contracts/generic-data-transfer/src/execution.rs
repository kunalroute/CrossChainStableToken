use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use router_wasm_bindings::RouterMsg;

use crate::{
    msg::ApplicationContract,
    print_debug_logs,
    query::fetch_white_listed_contract,
    state::{OWNER, WHITE_LISTED_CONTRACTS},
};

pub fn white_list_application_contract(
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
        .add_attribute("application", hex::encode(contract_address));
    Ok(res)
}

pub fn delist_application_contract(
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

pub fn update_white_listed_application_contracts(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contracts: Vec<ApplicationContract>,
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
        "update_destinstion_contract_address-- contracts: {:?}, sender: {:?}",
        contracts,
        info.sender.to_string()
    );
    print_debug_logs(deps.branch(), &env, &info_str);

    match fetch_white_listed_contract(deps.as_ref()) {
        Ok(keys) => {
            for i in 0..keys.len() {
                WHITE_LISTED_CONTRACTS
                    .remove(deps.storage, (keys[i].0.clone(), &keys[i].1, keys[i].2))
            }
        }
        Err(err) => return Err(err),
    }
    for i in 0..contracts.len() {
        WHITE_LISTED_CONTRACTS.save(
            deps.storage,
            (
                contracts[i].contract_address.clone(),
                &contracts[i].chain_id,
                contracts[i].chain_type,
            ),
            &true,
        )?;
    }

    let res = Response::new();
    Ok(res)
}
