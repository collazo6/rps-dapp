#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Api, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{GameResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, GameMove, GameData, GAMEDATA, GameResult};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Instantiate smart contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
)
}

// Initialize execute functionality.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartGame { opponent, host_move } => try_start_game(deps, info, opponent, host_move),
    }
}

// Attempts to start a new game.
pub fn try_start_game(deps: DepsMut, info: MessageInfo, opponent: Addr, host_move: GameMove) -> Result<Response, ContractError> {
    
    // Ensure host does not have another game in session.
    if GAMEDATA.may_load(deps.storage, &info.sender)? != None {
        return Err(ContractError::GameInSession {});
    }

    // Create game data and store in map.
    let game_data = GameData {
        host: info.sender,
        opponent,
        host_move: host_move,
        opp_move: GameMove::Waiting,
        result: GameResult::InProgress
    };
    GAMEDATA.save(deps.storage, &game_data.host, &game_data)?;

    Ok(Response::new()
        .add_attribute("method", "try_upsert_entry")
    )
}

// Initialize query message functionality.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGame { host } => to_binary(&query_game(deps, host)?),
    }
}

fn query_game(deps: Deps, host: Addr) -> StdResult<GameResponse> {
    let game_data = GAMEDATA.load(deps.storage, &host)?;
    Ok(GameResponse { game: game_data })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {

        // Create relevant variables for testing.
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // Ensure proper functionality of initialization of contract.
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(deps.api.addr_validate("creator").unwrap(), state.owner);
    }

    #[test]
    fn create_game() {

        // Instantiate contract.
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {  };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Ensure sender may create a new game.
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::StartGame { opponent: deps.api.addr_validate("someone").unwrap(), host_move: GameMove::Rock };
        let _res = execute(deps.as_mut(), mock_env(), info, msg);
    
        // Ensure game created as expected.
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGame { 
            host: deps.api.addr_validate("creator").unwrap()
         }).unwrap();
        let value: GameResponse = from_binary(&res).unwrap();
        let expected_output = GameResponse{ game: GameData {
            host: deps.api.addr_validate("creator").unwrap(),
            opponent: deps.api.addr_validate("someone").unwrap(),
            host_move: GameMove::Rock,
            opp_move: GameMove::Waiting,
            result: GameResult::InProgress
        }};
        assert_eq!(expected_output, value);

        // Ensure sender may only create one game at a time.
        let unauth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::StartGame { opponent: deps.api.addr_validate("someone").unwrap(), host_move: GameMove::Rock };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::GameInSession {}) => {}
            _ => panic!("Must return game in session error"),
        }
    }
}
