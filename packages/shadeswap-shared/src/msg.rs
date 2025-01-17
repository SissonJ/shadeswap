use fadroma::{
    scrt::{Binary, Decimal, HumanAddr, Uint128},
    scrt_callback::Callback,
    scrt_link::{ContractInstantiationInfo, ContractLink},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::TokenType;

pub use crate::snip20_impl::msg as snip20;
use crate::token_amount::TokenAmount;
use crate::token_pair_amount::TokenPairAmount;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

pub mod router {

    use fadroma::ViewingKey;

    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub enum InvokeMsg {
        SwapTokensForExact {
            paths: Vec<HumanAddr>,
            expected_return: Option<Uint128>,
            recipient: Option<HumanAddr>
        },
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct InitMsg {
        pub factory_address: ContractLink<HumanAddr>,
        pub prng_seed: Binary,
        pub entropy: Binary,
        pub viewing_key: Option<ViewingKey>
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum HandleMsg {
        // SNIP20 receiver interface
        Receive {
            from: HumanAddr,
            msg: Option<Binary>,
            amount: Uint128,
        },
        SwapTokensForExact {
            /// The token type to swap from.
            offer: TokenAmount<HumanAddr>,
            expected_return: Option<Uint128>,
            path: Vec<HumanAddr>,
            recipient: Option<HumanAddr>
        },
        SwapCallBack {
            last_token_out: TokenAmount<HumanAddr>,
            signature: Binary,
        },
        RegisterSNIP20Token{
            token: HumanAddr,
            token_code_hash: String
        }
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
    }
}

pub mod amm_pair {
    use super::*;
    use crate::{amm_pair::AMMSettings, fadroma::HumanAddr, Pagination, TokenPair, stake_contract::StakingContractInit};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize,  PartialEq, Debug, JsonSchema)]
    pub struct SwapInfo {
        pub total_fee_amount: Uint128,
        pub lp_fee_amount: Uint128,
        pub shade_dao_fee_amount: Uint128,
        pub result: SwapResult,
        pub price: Uint128
    }
    
    #[derive(Serialize, Deserialize,  PartialEq, Debug, JsonSchema)]
    pub struct SwapResult {
        pub return_amount: Uint128,
        pub spread_amount: Uint128,
    } 
    
    #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, JsonSchema)]
    pub struct TradeHistory {
        pub price: Uint128,
        pub amount: Uint128,
        pub timestamp: u64,
        pub direction: String,
        pub total_fee_amount: Uint128,
        pub lp_fee_amount: Uint128,
        pub shade_dao_fee_amount: Uint128,
    }
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct InitMsg {
        pub pair: TokenPair<HumanAddr>,
        pub lp_token_contract: ContractInstantiationInfo,
        pub factory_info: ContractLink<HumanAddr>,
        pub prng_seed: Binary,
        pub callback: Option<Callback<HumanAddr>>,
        pub entropy: Binary,
        pub admin: Option<HumanAddr>,
        pub staking_contract: Option<StakingContractInit>
    }
    #[derive(Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum HandleMsg {
        AddLiquidityToAMMContract {
            deposit: TokenPairAmount<HumanAddr>,
            slippage: Option<Decimal>,
        },
        SwapTokens {
            /// The token type to swap from.
            offer: TokenAmount<HumanAddr>,
            expected_return: Option<Uint128>,
            to: Option<HumanAddr>,
            router_link: Option<ContractLink<HumanAddr>>,
            callback_signature: Option<Binary>
        },
        // SNIP20 receiver interface
        Receive {
            from: HumanAddr,
            msg: Option<Binary>,
            amount: Uint128,
        },
        // Sent by the LP token contract so that we can record its address.
        OnLpTokenInitAddr,
        AddWhiteListAddress {
            address: HumanAddr,
        },
        RemoveWhitelistAddresses {
            addresses: Vec<HumanAddr>
        },
        SetAMMPairAdmin {
            admin: HumanAddr
        },
        SetStakingContract { contract: ContractLink<HumanAddr> },
    }
    #[derive(Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum InvokeMsg {
        SwapTokens {
            expected_return: Option<Uint128>,
            to: Option<HumanAddr>,
            router_link: Option<ContractLink<HumanAddr>>,
            callback_signature: Option<Binary>
        },
        RemoveLiquidity {
            recipient: HumanAddr,
        },
    }
    #[derive(Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        GetPairInfo,
        GetTradeHistory { pagination: Pagination },
        GetWhiteListAddress,
        GetTradeCount,
        GetAdmin,
        GetStakingContract,
        GetClaimReward{time: u128, staker: HumanAddr},
        GetEstimatedPrice { offer: TokenAmount<HumanAddr>}
    }
    #[derive(Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsgResponse {
        GetPairInfo {
            liquidity_token: ContractLink<HumanAddr>,
            factory: ContractLink<HumanAddr>,
            pair: TokenPair<HumanAddr>,
            amount_0: Uint128,
            amount_1: Uint128,
            total_liquidity: Uint128,
            contract_version: u32,
        },
        GetTradeHistory {
            data: Vec<TradeHistory>,
        },
        GetWhiteListAddress {
            addresses: Vec<HumanAddr>,
        },
        GetTradeCount {
            count: u64,
        },
        GetAdminAddress {
            address: HumanAddr
        },
        GetClaimReward {
            amount: Uint128,
        },
        StakingContractInfo{
            staking_contract: ContractLink<HumanAddr>
        },
        EstimatedPrice {
            estimated_price: Uint128
        }
    }
}

pub mod factory {
    use crate::{amm_pair::AMMSettings, fadroma::HumanAddr, Pagination, TokenPair};
    use fadroma::{Binary, ContractInstantiationInfo};
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use crate::amm_pair::{{AMMPair}};
    use crate::stake_contract::StakingContractInit;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct InitMsg {
        pub pair_contract: ContractInstantiationInfo,
        pub amm_settings: AMMSettings<HumanAddr>,
        pub lp_token_contract: ContractInstantiationInfo,
        pub prng_seed: Binary,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum HandleMsg {
        SetConfig {
            pair_contract: Option<ContractInstantiationInfo>,
            lp_token_contract: Option<ContractInstantiationInfo>,
            amm_settings: Option<AMMSettings<HumanAddr>>,
        },
        CreateAMMPair {
            pair: TokenPair<HumanAddr>,
            entropy: Binary,
            staking_contract: Option<StakingContractInit>
        },
        AddAMMPairs {
            amm_pairs: Vec<AMMPair<HumanAddr>>,
        },
        RegisterAMMPair {
            pair: TokenPair<HumanAddr>,
            signature: Binary,
        },
        SetFactoryAdmin {
            admin: HumanAddr
        }
    }

    #[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryResponse {
        ListAMMPairs {
            amm_pairs: Vec<AMMPair<HumanAddr>>,
        },
        GetConfig {
            pair_contract: ContractInstantiationInfo,
            amm_settings: AMMSettings<HumanAddr>,
            lp_token_contract: ContractInstantiationInfo,
        },
        GetAMMPairAddress {
            address: HumanAddr,
        },
        GetAMMSettings {
            settings: AMMSettings<HumanAddr>,
        },
        GetAdminAddress {
            address: HumanAddr
        },        
    }

    #[derive(Serialize, Deserialize, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        // GetCount returns the current count as a json-encoded number
        ListAMMPairs { pagination: Pagination },
        GetAMMPairAddress { pair: TokenPair<HumanAddr> },
        GetAMMSettings,
        GetConfig,
        GetAdmin
    }
}

pub mod staking {
    use super::*;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct InitMsg {
        pub staking_amount: Uint128,
        pub reward_token: TokenType<HumanAddr>, 
        pub contract: ContractLink<HumanAddr>
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum HandleMsg {
        ClaimRewards {}, 
        Stake {   
            from: HumanAddr,
            amount: Uint128,
        },
        Unstake {
            address: HumanAddr
        },  
    }

    #[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        GetStakers {},
        GetClaimReward {time: u128, staker: HumanAddr},
        GetContractOwner {}
    }

    #[derive(Serialize, Deserialize, Debug, JsonSchema, PartialEq)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryResponse {
        Stakers {
            stakers: Vec<HumanAddr>
        },
        ClaimReward {
            amount: Uint128
        },
        ContractOwner {
            address: HumanAddr
        }
    }

}