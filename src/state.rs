use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub denom: String,
    pub price: Uint128,
    pub decimals: Uint128,
    pub config_set: bool,
    pub max_mint: Uint128,
    pub withdraw_flag: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const PROTOCOL_TOKEN: Item<Addr> = Item::new("protocol Token");

pub const TOTAL_DEPOSIT: Item<Uint128> = Item::new("total_deposit");
