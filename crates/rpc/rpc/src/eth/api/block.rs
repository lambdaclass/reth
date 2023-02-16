//! Contains RPC handler implementations specific to blocks.

use crate::{eth::error::EthResult, EthApi};
use reth_primitives::{rpc::BlockId, H256, U256};
use reth_provider::{BlockProvider, StateProviderFactory};
use reth_rpc_types::{RichBlock, Block};
use std::collections::BTreeMap;

impl<Client, Pool, Network> EthApi<Client, Pool, Network>
where
    Client: BlockProvider + StateProviderFactory + 'static,
{
    pub(crate) async fn block_by_hash(
        &self,
        hash: H256,
        _full: bool,
    ) -> EthResult<Option<RichBlock>> {
        let hash = BlockId::Hash(hash.0.into());
        println!("Hash's value: {:?}", hash);
        let block = self.client().block(hash)?;
        println!("{:?}", block);
        if let Some(block) = block {
            // TODO: GET TD FOR BLOCK - needs block provider? or header provider?
            // let total_difficulty = todo!();
            // let rich_block = Block::from_block_full(block, total_difficulty);
            let full_block = Block::from_block_full(block, U256::from(0)).unwrap();
            Ok(Some(RichBlock {inner: full_block, extra_info: BTreeMap::new()}))
            // todo!()
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn block_by_number(
        &self,
        number: u64,
        _full: bool,
    ) -> EthResult<Option<RichBlock>> {
        let block = self.client().block(BlockId::Number(number.into()))?;
        if let Some(_block) = block {
            // TODO: GET TD FOR BLOCK - needs block provider? or header provider?
            // let total_difficulty = todo!();
            // let rich_block = Block::from_block_full(block, total_difficulty);
            todo!()
        } else {
            Ok(None)
        }
    }
}
