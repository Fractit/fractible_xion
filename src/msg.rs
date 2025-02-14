use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub denom: String,
    pub price: Uint128,
    pub decimals: Uint128,
    pub max_mint: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig { protocol_token: Addr },
    Deposit {},
    Withdraw { amount: Uint128 },
    Claim {},
    PauseWithdraw { flag: bool },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetTotalDepositResponse)]
    GetTotalDeposit {},
    #[returns(GetConfigResponse)]
    GetConfig {},
}

#[cw_serde]
pub struct GetTotalDepositResponse {
    pub total_deposit: Uint128,
}

#[cw_serde]
pub struct GetConfigResponse {
    pub owner: String,
    pub denom: String,
    pub token: String,
    pub withdraw_flag: bool,
}
