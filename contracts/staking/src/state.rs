use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Decimal, HumanAddr, Uint128};
use cosmwasm_storage::{bucket, singleton, Bucket, Singleton};

use crate::msg::TokenInfoResponse;

pub const KEY_INVESTMENT: &[u8] = b"invest";
pub const KEY_TOKEN_INFO: &[u8] = b"token";
pub const KEY_TOTAL_SUPPLY: &[u8] = b"total_supply";

pub const PREFIX_BALANCE: &[u8] = b"balance";
pub const PREFIX_CLAIMS: &[u8] = b"claim";

/// balances are state of the erc20 tokens
pub fn balances() -> Bucket<Uint128> {
    bucket(PREFIX_BALANCE)
}

/// claims are the claims to money being unbonded
pub fn claims() -> Bucket<Uint128> {
    bucket(PREFIX_CLAIMS)
}

/// Investment info is fixed at initialization, and is used to control the function of the contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InvestmentInfo {
    /// owner created the contract and takes a cut
    pub owner: CanonicalAddr,
    /// this is the denomination we can stake (and only one we accept for payments)
    pub bond_denom: String,
    /// this is how much the owner takes as a cut when someone unbonds
    pub exit_tax: Decimal,
    /// All tokens are bonded to this validator
    /// FIXME: humanize/canonicalize address doesn't work for validator addrresses
    pub validator: HumanAddr,
    /// This is the minimum amount we will pull out to reinvest, as well as a minumum
    /// that can be unbonded (to avoid needless staking tx)
    pub min_withdrawal: Uint128,
}

/// Supply is dynamic and tracks the current supply of staked and ERC20 tokens.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct Supply {
    /// issued is how many derivative tokens this contract has issued
    pub issued: Uint128,
    /// bonded is how many native tokens exist bonded to the validator
    pub bonded: Uint128,
    /// claims is how many tokens need to be reserved paying back those who unbonded
    pub claims: Uint128,
}

pub fn invest_info() -> Singleton<InvestmentInfo> {
    singleton(KEY_INVESTMENT)
}

pub fn token_info() -> Singleton<TokenInfoResponse> {
    singleton(KEY_TOKEN_INFO)
}

pub fn total_supply() -> Singleton<Supply> {
    singleton(KEY_TOTAL_SUPPLY)
}
