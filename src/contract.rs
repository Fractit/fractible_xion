#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_json_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, WasmMsg,
};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, GetConfigResponse, GetTotalDepositResponse, InstantiateMsg, QueryMsg,
};
use crate::state::{Config, CONFIG, PROTOCOL_TOKEN, TOTAL_DEPOSIT};
use cosmwasm_std::{Addr, BankMsg, Uint128};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:newc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let valid_owner = deps.api.addr_validate(&msg.owner)?;

    let new_config = Config {
        owner: valid_owner,
        denom: msg.denom,
        price: msg.price,
        decimals: msg.decimals,
        config_set: false,
        max_mint: msg.max_mint,
        withdraw_flag: false,
    };

    CONFIG.save(deps.storage, &new_config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetConfig { protocol_token } => {
            execute_set_config(deps, env, info, protocol_token)
        }
        ExecuteMsg::Deposit {} => execute_deposit(deps, env, info),
        ExecuteMsg::Withdraw { amount } => execute_withdraw(deps, env, info, amount),
        ExecuteMsg::Claim {} => execute_claim(deps, env, info),
        ExecuteMsg::PauseWithdraw { flag } => execute_pause_withdraw(deps, env, info, flag),
    }
}

fn execute_set_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    protocol_token: Addr,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::OnlyOwner {});
    }
    if config.config_set {
        return Err(ContractError::ConfigSetAlready {});
    }
    PROTOCOL_TOKEN.save(deps.storage, &protocol_token)?;
    config.config_set = true;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new())
}

fn execute_pause_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    flag: bool,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let sender = info.sender;

    if sender != config.owner {
        return Err(ContractError::OnlyOwner {});
    }

    config.withdraw_flag = flag;

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("ExecutePauseWithdraw", flag.to_string()))
}

fn execute_deposit(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let token = PROTOCOL_TOKEN.load(deps.storage)?;
    let sent_fund = info.funds;
    let mut total_deposit = TOTAL_DEPOSIT.load(deps.storage).unwrap_or_default();

    let amount = amount_sent(sent_fund, config.denom.clone());

    total_deposit += amount;

    if total_deposit > config.max_mint {
        return Err(ContractError::DepositMoreThanMax {});
    }
    TOTAL_DEPOSIT.save(deps.storage, &total_deposit)?;

    let mint_msg = mint_tokens(info.sender, amount, token);

    Ok(Response::new().add_message(mint_msg))
}

fn execute_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let token = PROTOCOL_TOKEN.load(deps.storage)?;

    if !config.withdraw_flag {
        return Err(ContractError::WithdrawPause {});
    }

    let burn_msg = burn_tokens(info.sender.clone(), amount, token);
    let send_msg = send_native(info.sender, amount, config.denom);
    Ok(Response::new().add_message(burn_msg).add_message(send_msg))
}

fn execute_claim(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let balance = contract_balance(deps.as_ref(), env, config.denom.clone());

    if info.sender != config.owner {
        return Err(ContractError::OnlyOwner {});
    }

    let send_msg = send_native(config.owner, balance, config.denom.clone());

    Ok(Response::new().add_message(send_msg))
}

fn amount_sent(sent_funds: Vec<Coin>, denom: String) -> Uint128 {
    let amount = sent_funds
        .iter()
        .find(|coin| coin.denom == denom)
        .map(|coin| coin.amount)
        .unwrap_or(Uint128::zero());
    return amount;
}

fn send_native(recipient: Addr, amount: Uint128, denom: String) -> CosmosMsg {
    let send_msg = BankMsg::Send {
        to_address: recipient.clone().to_string(),
        amount: vec![Coin {
            denom: denom,
            amount: amount,
        }],
    };

    let msg: CosmosMsg = send_msg.into();
    return msg;
}

fn mint_tokens(recipient: Addr, mint_amount: Uint128, token: Addr) -> CosmosMsg {
    let mint_msg = cw20_base::msg::ExecuteMsg::Mint {
        recipient: recipient.into(),
        amount: Uint128::from(mint_amount),
    };

    let msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_json_binary(&mint_msg).unwrap(),
        funds: vec![],
    }
    .into();

    return msg;
}

fn burn_tokens(user: Addr, burn_amount: Uint128, token: Addr) -> CosmosMsg {
    let burn_msg = cw20_base::msg::ExecuteMsg::BurnFrom {
        owner: user.into(),
        amount: burn_amount,
    };

    let msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: token.to_string(),
        msg: to_json_binary(&burn_msg).unwrap(),
        funds: vec![],
    }
    .into();

    return msg;
}
fn contract_balance(deps: Deps, env: Env, denom: String) -> Uint128 {
    let balance = deps
        .querier
        .query_balance(env.contract.address, denom)
        .unwrap();
    balance.amount
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_get_config(deps)?),
        QueryMsg::GetTotalDeposit {} => to_json_binary(&query_get_total_deposit(deps, env)?),
    }
}

fn query_get_config(deps: Deps) -> StdResult<GetConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let token = PROTOCOL_TOKEN.load(deps.storage)?;
    Ok(GetConfigResponse {
        owner: config.owner.to_string(),
        denom: config.denom,
        token: token.into_string(),
        withdraw_flag: config.withdraw_flag,
    })
}

fn query_get_total_deposit(deps: Deps, env: Env) -> StdResult<GetTotalDepositResponse> {
    let config = CONFIG.load(deps.storage)?;
    let contract_balance = contract_balance(deps, env, config.denom.clone());
    Ok(GetTotalDepositResponse {
        total_deposit: contract_balance,
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Addr, Coin, Empty, Uint128};
    use cw20_base::contract;
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use super::*;

    fn cw20_token() -> Box<dyn Contract<Empty>> {
        let cw20_contract =
            ContractWrapper::new(contract::execute, contract::instantiate, contract::query);
        Box::new(cw20_contract)
    }

    fn protocol_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    fn deploy_protcol(protocol_id: u64, app: &mut App, sender: Addr) -> Addr {
        let contract_addrss = app
            .instantiate_contract(
                protocol_id,
                sender.clone(),
                &InstantiateMsg {
                    owner: sender.to_string(),
                    denom: "usdc".to_string(),
                    price: Uint128::one(),
                    decimals: Uint128::from(6u64),
                    max_mint: Uint128::from(30000000000u128),
                },
                &[],
                "FractIt01",
                None,
            )
            .unwrap();

        return contract_addrss;
    }

    fn deploy_cw20_contract(
        cw20_id: u64,
        protocol_addr: Addr,
        app: &mut App,
        sender: Addr,
    ) -> Addr {
        let contract_addrss = app
            .instantiate_contract(
                cw20_id,
                sender.clone(),
                &cw20_base::msg::InstantiateMsg {
                    name: "Fractible ST3004".into(),
                    symbol: "FSTTHREE".into(),
                    decimals: 6,
                    initial_balances: vec![],
                    mint: Some(cw20::MinterResponse {
                        minter: protocol_addr.to_string(),
                        cap: None,
                    }),
                    marketing: None,
                },
                &[],
                "Cw20_token",
                None,
            )
            .unwrap();

        return contract_addrss;
    }

    fn mint_native(app: &mut App, recipient: String, denom: String, amount: u128) {
        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: recipient,
                amount: vec![coin(amount, denom)],
            },
        ))
        .unwrap();
    }

    fn deploy_all_contracts(mut app: App) -> (App, Addr, Addr) {
        // let owner_addr = Addr::unchecked("owner");
        let owner_addr = app.api().addr_make("owner");

        let cw20_id = app.store_code(cw20_token());

        let protocol_id = app.store_code(protocol_contract());

        mint_native(
            &mut app,
            owner_addr.to_string(),
            "usdc".to_string(),
            1_100_000u128,
        );

        let protocol_addr = deploy_protcol(protocol_id, &mut app, owner_addr.clone());

        println!("contract protocol is {}", protocol_addr.clone());

        let fracit_cw20 =
            deploy_cw20_contract(cw20_id, protocol_addr.clone(), &mut app, owner_addr.clone());

        println!("contract cw20 is {}", fracit_cw20.clone());

        let execut_msg = ExecuteMsg::SetConfig {
            protocol_token: fracit_cw20.clone(),
        };

        let response = app
            .execute_contract(owner_addr.clone(), protocol_addr.clone(), &execut_msg, &[])
            .unwrap();
        return (app, protocol_addr, fracit_cw20);
    }

    fn get_cw20_balance(owner_addr: Addr, app: App, contract_addrss: Addr) -> (Uint128, App) {
        let mut qur_msg = cw20_base::msg::QueryMsg::Balance {
            address: owner_addr.to_string(),
        };

        let mut qry_res: cw20::BalanceResponse = app
            .wrap()
            .query_wasm_smart(contract_addrss, &qur_msg)
            .unwrap();

        return (qry_res.balance, app);
    }
    #[test]

    fn test_deployment() {
        let oldapp = App::default();

        let user_addr = oldapp.api().addr_make("user");
        let owner_addr = oldapp.api().addr_make("owner");

        let (mut app, protocol_addrss, fractit_token) = deploy_all_contracts(oldapp);

        let (mut balance, mut app) =
            get_cw20_balance(user_addr.clone(), app, fractit_token.clone());

        let query_config = QueryMsg::GetConfig {};

        let get_config: GetConfigResponse = app
            .wrap()
            .query_wasm_smart(protocol_addrss.clone(), &query_config)
            .unwrap();

        println!("query response is {:?}", get_config);

        println!("before balance is{}", balance);

        let deposit_msg = ExecuteMsg::Deposit {};

        let deposit_response = app
            .execute_contract(
                owner_addr.clone(),
                protocol_addrss.clone(),
                &deposit_msg,
                &vec![coin(100, "usdc")],
            )
            .unwrap();

        let (mut balance, mut app) =
            get_cw20_balance(owner_addr.clone(), app, fractit_token.clone());

        println!("before after deposit is{}", balance);

        let mut contract_balance_msg = QueryMsg::GetTotalDeposit {};

        let mut deposit_response: GetTotalDepositResponse = app
            .wrap()
            .query_wasm_smart(protocol_addrss.clone(), &contract_balance_msg)
            .unwrap();

        println!("Deposit repsonse is {:?}", deposit_response);

        let allowance_msg = cw20_base::msg::ExecuteMsg::IncreaseAllowance {
            spender: protocol_addrss.clone().into(),
            amount: Uint128::from(50u128),
            expires: None,
        };

        let allowance_response = app
            .execute_contract(
                owner_addr.clone(),
                fractit_token.clone(),
                &allowance_msg,
                &[],
            )
            .unwrap();

        let withdraw_msg = ExecuteMsg::Withdraw {
            amount: Uint128::from(50u128),
        };

        let withdraw_response = app
            .execute_contract(
                owner_addr.clone(),
                protocol_addrss.clone(),
                &withdraw_msg,
                &vec![],
            )
            .unwrap();

        let (mut balance, mut app) =
            get_cw20_balance(owner_addr.clone(), app, fractit_token.clone());

        println!("before after withdraw is{}", balance);

        let mut balances = app.wrap().query_all_balances(&owner_addr).unwrap();

        for coin in balances {
            println!("beofre balance is {}: {}", coin.denom, coin.amount);
        }

        let claim_msg = ExecuteMsg::Claim {};

        let claim_response = app
            .execute_contract(owner_addr.clone(), protocol_addrss.clone(), &claim_msg, &[])
            .unwrap();

        let mut balances = app.wrap().query_all_balances(&owner_addr).unwrap();

        for coin in balances {
            println!("beofre balance is {}: {}", coin.denom, coin.amount);
        }

        let mut contract_balance_msg = QueryMsg::GetTotalDeposit {};

        let mut deposit_response: GetTotalDepositResponse = app
            .wrap()
            .query_wasm_smart(protocol_addrss.clone(), &contract_balance_msg)
            .unwrap();

        println!("contract balance after claim is {:?}", deposit_response);
    }
}
