// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumBlockEvents {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<RaydiumTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<RaydiumEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumEvent {
    #[prost(oneof="raydium_event::Event", tags="1, 2, 3, 4, 5")]
    pub event: ::core::option::Option<raydium_event::Event>,
}
/// Nested message and enum types in `RaydiumEvent`.
pub mod raydium_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Initialize(super::InitializeEvent),
        #[prost(message, tag="2")]
        Deposit(super::DepositEvent),
        #[prost(message, tag="3")]
        Withdraw(super::WithdrawEvent),
        #[prost(message, tag="4")]
        WithdrawPnl(super::WithdrawPnlEvent),
        #[prost(message, tag="5")]
        Swap(super::SwapEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub pc_init_amount: u64,
    #[prost(uint64, tag="4")]
    pub coin_init_amount: u64,
    #[prost(uint64, tag="5")]
    pub lp_init_amount: u64,
    #[prost(string, tag="6")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub lp_mint: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub nonce: u32,
    #[prost(string, optional, tag="10")]
    pub market: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub pc_amount: u64,
    #[prost(uint64, tag="4")]
    pub coin_amount: u64,
    #[prost(uint64, tag="5")]
    pub lp_amount: u64,
    #[prost(string, tag="6")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub lp_mint: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="9")]
    pub pool_pc_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="10")]
    pub pool_coin_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="11")]
    pub pool_lp_amount: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub pc_amount: u64,
    #[prost(uint64, tag="4")]
    pub coin_amount: u64,
    #[prost(uint64, tag="5")]
    pub lp_amount: u64,
    #[prost(string, tag="6")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub lp_mint: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="9")]
    pub pool_pc_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="10")]
    pub pool_coin_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="11")]
    pub pool_lp_amount: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawPnlEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="3")]
    pub pc_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub coin_amount: ::core::option::Option<u64>,
    #[prost(string, optional, tag="6")]
    pub pc_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="7")]
    pub coin_mint: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint_in: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub mint_out: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub amount_in: u64,
    #[prost(uint64, tag="6")]
    pub amount_out: u64,
    #[prost(string, tag="7")]
    pub direction: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="8")]
    pub pool_pc_amount: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="9")]
    pub pool_coin_amount: ::core::option::Option<u64>,
    #[prost(string, tag="10")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub coin_mint: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
