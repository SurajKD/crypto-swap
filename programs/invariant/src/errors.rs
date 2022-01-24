use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Amount is zero")]
    ZeroAmount = 0, // 1770
    #[msg("Output would be zero")]
    ZeroOutput = 1, // 1771
    #[msg("Not the expected tick")]
    WrongTick = 2, // 1772
    #[msg("Price limit is on the wrong side of price")]
    WrongLimit = 3, // 1773
    #[msg("Tick index not divisible by spacing or over limit")]
    InvalidTickIndex = 4, // 1774
    #[msg("Invalid tick_lower or tick_upper")]
    InvalidTickInterval = 5, // 1775
    #[msg("There is no more tick in that direction")]
    NoMoreTicks = 6, // 1776
    #[msg("Correct tick not found in context")]
    TickNotFound = 7, // 1777
    #[msg("Price would cross swap limit")]
    PriceLimitReached = 8, // 1778
    #[msg("Invalid tick liquidity")]
    InvalidTickLiquidity = 9, // 1779
    #[msg("Disable empty position pokes")]
    EmptyPositionPokes = 10, // 177a
    #[msg("Invalid tick liquidity")]
    InvalidPositionLiquidity = 11, // 177b
    #[msg("Invalid pool liquidity")]
    InvalidPoolLiquidity = 12, // 177c
    #[msg("Invalid position index")]
    InvalidPositionIndex = 13, // 177d
    #[msg("Position liquidity would be zero")]
    PositionWithoutLiquidity = 14, // 177e
    #[msg("You are not admin")]
    Unauthorized = 15, // 177f
    #[msg("Invalid pool token addresses")]
    InvalidPoolTokenAddresses = 16, // 1780
    #[msg("Time cannot be negative")]
    NegativeTime = 17, // 1781
    #[msg("Oracle is already initialized")]
    OracleAlreadyInitialized = 18, // 1782
    #[msg("Absolute price limit was reached")]
    LimitReached = 19, // 1783
    #[msg("Invalid protocol fee")]
    InvalidProtocolFee = 20, // 1784
}
