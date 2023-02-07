//! Reth block execution/validation configuration and constants

use reth_primitives::{BlockNumber, ChainSpec, Hardfork};

/// Two ethereum worth of wei
pub const WEI_2ETH: u128 = 2000000000000000000u128;
/// Three ethereum worth of wei
pub const WEI_3ETH: u128 = 3000000000000000000u128;
/// Five ethereum worth of wei
pub const WEI_5ETH: u128 = 5000000000000000000u128;

use revm::primitives::SpecId;
/// return revm_spec from spec configuration.
pub fn revm_spec(chain_spec: &ChainSpec, for_block: BlockNumber) -> revm::primitives::SpecId {
    match for_block {
        b if chain_spec.fork_active(Hardfork::Shanghai, b) => SpecId::MERGE_EOF,
        b if Some(b) >= chain_spec.paris_status().block_number() => SpecId::MERGE,
        b if chain_spec.fork_active(Hardfork::London, b) => SpecId::LONDON,
        b if chain_spec.fork_active(Hardfork::Berlin, b) => SpecId::BERLIN,
        b if chain_spec.fork_active(Hardfork::Istanbul, b) => SpecId::ISTANBUL,
        b if chain_spec.fork_active(Hardfork::Petersburg, b) => SpecId::PETERSBURG,
        b if chain_spec.fork_active(Hardfork::Byzantium, b) => SpecId::BYZANTIUM,
        b if chain_spec.fork_active(Hardfork::SpuriousDragon, b) => SpecId::SPURIOUS_DRAGON,
        b if chain_spec.fork_active(Hardfork::Tangerine, b) => SpecId::TANGERINE,
        b if chain_spec.fork_active(Hardfork::Homestead, b) => SpecId::HOMESTEAD,
        b if chain_spec.fork_active(Hardfork::Frontier, b) => SpecId::FRONTIER,
        _ => panic!("wrong configuration"),
    }
}

#[cfg(test)]
mod tests {
    use crate::config::revm_spec;
    use reth_primitives::{ChainSpecBuilder, MAINNET};
    #[test]
    fn test_to_revm_spec() {
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().paris_activated().build(), 1),
            revm::primitives::MERGE
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().london_activated().build(), 1),
            revm::primitives::LONDON
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().berlin_activated().build(), 1),
            revm::primitives::BERLIN
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().istanbul_activated().build(), 1),
            revm::primitives::ISTANBUL
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().petersburg_activated().build(), 1),
            revm::primitives::PETERSBURG
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().byzantium_activated().build(), 1),
            revm::primitives::BYZANTIUM
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().spurious_dragon_activated().build(), 1),
            revm::primitives::SPURIOUS_DRAGON
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().tangerine_whistle_activated().build(), 1),
            revm::primitives::TANGERINE
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().homestead_activated().build(), 1),
            revm::primitives::HOMESTEAD
        );
        assert_eq!(
            revm_spec(&ChainSpecBuilder::mainnet().frontier_activated().build(), 1),
            revm::primitives::FRONTIER
        );
    }

    #[test]
    fn test_eth_spec() {
        assert_eq!(revm_spec(&MAINNET, 15537394 + 10), revm::primitives::MERGE);
        assert_eq!(revm_spec(&MAINNET, 15537394 - 10), revm::primitives::LONDON);
        assert_eq!(revm_spec(&MAINNET, 12244000 + 10), revm::primitives::BERLIN);
        assert_eq!(revm_spec(&MAINNET, 12244000 - 10), revm::primitives::ISTANBUL);
        assert_eq!(revm_spec(&MAINNET, 7280000 + 10), revm::primitives::PETERSBURG);
        assert_eq!(revm_spec(&MAINNET, 7280000 - 10), revm::primitives::BYZANTIUM);
        assert_eq!(revm_spec(&MAINNET, 2675000 + 10), revm::primitives::SPURIOUS_DRAGON);
        assert_eq!(revm_spec(&MAINNET, 2675000 - 10), revm::primitives::TANGERINE);
        assert_eq!(revm_spec(&MAINNET, 1150000 + 10), revm::primitives::HOMESTEAD);
        assert_eq!(revm_spec(&MAINNET, 1150000 - 10), revm::primitives::FRONTIER);
    }
}
