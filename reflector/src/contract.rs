use cosmwasm_std::{
    attr, entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    SubMsg, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StateInfoResponse};
use crate::state::{State, STATE};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
        reflectee: info.sender, // placeholder until we set the reflectee properly...
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetReflectee { reflectee } => try_set_reflectee(deps, info, reflectee),
        ExecuteMsg::Reflect { msgs } => try_reflect(deps, info, msgs),
        ExecuteMsg::SendIncrementToReflectee {} => try_to_send_increment_to_reflectee(deps, info),
    }
}

pub fn try_set_reflectee(
    deps: DepsMut,
    info: MessageInfo,
    reflectee: String,
) -> Result<Response, ContractError> {
    let reflectee = deps.api.addr_validate(&reflectee)?;
    // load STATE
    let state = STATE.load(deps.storage)?;
    // Check that sender is the owner
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    // set the reflectee if the address is valid
    STATE.update(deps.storage, |mut config| -> StdResult<_> {
        config.reflectee = reflectee;
        Ok(config)
    })?;
    // return the standard Response
    let mut res = Response::default();
    res.add_attribute("action", "set_reflectee");
    Ok(res)
}

pub fn try_reflect(
    deps: DepsMut,
    info: MessageInfo,
    msgs: Vec<SubMsg>,
) -> Result<Response, ContractError> {
    // load STATE
    let state = STATE.load(deps.storage)?;
    // Check that sender is the owner
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    // make sure the messages vec has something in it
    if msgs.is_empty() {
        return Err(ContractError::EmptyMessages {});
    }
    // send the passed messages on to be executed on the reflectee SC address
    let res = Response {
        messages: msgs,
        attributes: vec![attr("action", "reflect")],
        ..Response::default()
    };
    Ok(res)
}

pub fn try_to_send_increment_to_reflectee(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // load STATE
    let state = STATE.load(deps.storage)?;
    // Check that sender is the owner
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    let sub_msg = SubMsg::new(WasmMsg::Execute {
        contract_addr: state.reflectee.to_string(),
        msg: Binary::from_base64("eyJpbmNyZW1lbnQiOnt9fQ==").unwrap(),
        funds: vec![],
    });

    // send the passed messages on to be executed on the reflectee SC address
    let res = Response {
        messages: vec![sub_msg],
        attributes: vec![attr("action", "send_increment_to_reflectee")],
        ..Response::default()
    };
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetInfo {} => to_binary(&query_info(deps)?),
    }
}

fn query_info(deps: Deps) -> StdResult<StateInfoResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(StateInfoResponse {
        owner: state.owner.to_string(),
        reflectee: state.reflectee.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        // setup addr for test
        let owner = String::from("owner");

        let mut deps = mock_dependencies(&[]);

        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(owner.as_ref(), &coins(100, "luna"));
        let env = mock_env();
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, now let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetInfo {}).unwrap();
        let value: StateInfoResponse = from_binary(&res).unwrap();
        // owner & reflectee should be set now & should both be the owner Addr
        assert_eq!(owner.clone(), value.owner);
        assert_eq!(owner.clone(), value.reflectee);
    }

    #[test]
    fn set_reflectee() {
        // setup addr for test
        let owner = String::from("owner");
        let sneaky_pleb = String::from("sneakypleb");

        let mut deps = mock_dependencies(&[]);

        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(owner.as_ref(), &coins(20, "luna"));
        let env = mock_env();
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // only owner can set reflectee
        let info = mock_info(sneaky_pleb.as_ref(), &coins(20, "luna"));
        let msg = ExecuteMsg::SetReflectee {
            reflectee: sneaky_pleb.clone(),
        };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // only owner can set reflectee
        let info = mock_info(owner.as_ref(), &coins(2, "luna"));
        let msg = ExecuteMsg::SetReflectee {
            reflectee: sneaky_pleb.clone(),
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, now let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetInfo {}).unwrap();
        let value: StateInfoResponse = from_binary(&res).unwrap();
        // owner & reflectee should be set, but the reflectee should have a different addr now
        assert_eq!(owner, value.owner);
        assert_eq!(sneaky_pleb, value.reflectee);
    }

    #[test]
    fn reflect() {
        // setup addr for test
        let owner = String::from("owner");
        let sneaky_pleb = String::from("sneakypleb");

        let mut deps = mock_dependencies(&[]);

        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(owner.as_ref(), &coins(20, "luna"));
        let env = mock_env();
        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // only owner can set reflect messages
        let info = mock_info(sneaky_pleb.as_ref(), &coins(20, "luna"));
        let msg = ExecuteMsg::Reflect { msgs: vec![] };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(err, ContractError::Unauthorized {});

        // empty messages should be rejected
        let info = mock_info(owner.as_ref(), &coins(20, "luna"));
        let msg = ExecuteMsg::Reflect { msgs: vec![] };
        let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        assert_eq!(err, ContractError::EmptyMessages {});

        // proper messages, sent by the owner can be Reflected to the reflectee
        let info = mock_info(owner.as_ref(), &coins(20, "luna"));
        let msg = ExecuteMsg::Reflect { msgs: vec![] };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
