//! Errors

use ethers::prelude::*;
use thiserror::Error;

/// Type alias for the results of [Dex].
///
/// [Dex]: crate::dex::Dex
pub type DexResult<T, M> = Result<T, DexError<M>>;

/// Errors thrown by [Dex].
///
/// [Dex]: crate::dex::Dex
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DexError<M: Middleware> {
    /// Thrown when using a library.
    #[error(transparent)]
    LibraryError(#[from] LibraryError),

    /// Thrown when interacting with the smart contracts.
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),

    /// Thrown when a provider call fails.
    #[error(transparent)]
    ProviderError(#[from] ProviderError),

    /// Thrown by the router.
    #[error(transparent)]
    RouterError(#[from] RouterError<M>),

    /// Thrown by the factory.
    #[error(transparent)]
    FactoryError(#[from] FactoryError<M>),

    /// Thrown by a pair.
    #[error(transparent)]
    PairError(#[from] PairError<M>),

    /// Thrown when the provided slippage is invalid.
    #[error("Slippage must be in range: 0.0..=100.0")]
    InvalidSlippage,

    /// Thrown when the start and finish token are the same.
    #[error("Cannot swap a token into itself")]
    SwapToSelf,

    /// Thrown when trying to create a WETH deposit or withdrawal and WETH has not been set.
    #[error("WETH has yet to be set")]
    WethNotSet,
}

/// Type alias for the results of a pair.
pub type PairResult<T, M> = Result<T, PairError<M>>;

/// Errors thrown by a pair.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PairError<M: Middleware> {
    /// Thrown when interacting with the smart contracts.
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),

    /// Thrown when using a library.
    #[error(transparent)]
    LibraryError(#[from] LibraryError),

    /// Thrown when interacting with [Multicall].
    #[error(transparent)]
    MulticallError(#[from] MulticallError<M>),
}

/// Type alias for the results of a router.
pub type RouterResult<T, M> = Result<T, RouterError<M>>;

/// Errors thrown by a router.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum RouterError<M: Middleware> {
    /// Thrown when interacting with the smart contracts.
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),

    /// Thrown when using a library.
    #[error(transparent)]
    LibraryError(#[from] LibraryError),
}

/// Type alias for the results of a factory.
pub type FactoryResult<T, M> = Result<T, FactoryError<M>>;

/// Errors thrown by a factory.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FactoryError<M: Middleware> {
    /// Thrown when interacting with the smart contracts.
    #[error(transparent)]
    ContractError(#[from] ContractError<M>),

    /// Thrown when using a library.
    #[error(transparent)]
    LibraryError(#[from] LibraryError),
}

/// Type alias for the results of a library.
pub type LibraryResult<T> = Result<T, LibraryError>;

/// Errors thrown by a library.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LibraryError {
    /// Thrown when interacting with the smart contracts.
    #[error("{0}")]
    ContractError(String),

    /// Thrown when interacting with [Multicall].
    #[error("{0}")]
    MulticallError(String),

    /// Thrown when providing identical addresses as parameters.
    #[error("Sorting identical addresses")]
    IdenticalAddresses,

    /// Thrown when providing [address(0)][Address] as a parameter.
    #[error("Sorting Address::zero()")]
    AddressZero,

    /// Thrown when providing an amount equal to zero.
    #[error("Amount is zero")]
    InsufficientAmount,

    /// Thrown when providing an input amount equal to zero.
    #[error("Input amount is zero")]
    InsufficientInputAmount,

    /// Thrown when providing an output amount equal to zero.
    #[error("Output amount is zero")]
    InsufficientOutputAmount,

    /// Thrown when providing a liquidity amount equal to zero.
    #[error("Liquidity is zero")]
    InsufficientLiquidity,

    /// Thrown when the provided path is empty or contains only one address.
    #[error("Path length must be greater than or equal to 2")]
    InvalidPath,

    /// Thrown when the factory provided returns none for pair_code_hash
    #[error("Custom protocol is missing pair_code_hash.")]
    NoPairCodeHash,
}
