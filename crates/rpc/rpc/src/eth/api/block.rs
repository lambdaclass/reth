//! Contains RPC handler implementations specific to blocks.
use crate::{
    eth::error::{EthApiError, EthResult},
    EthApi,
};
use reth_primitives::{rpc::BlockId, H256};
use reth_provider::{BlockProvider, HeaderProvider, StateProviderFactory};
use reth_rpc_types::{Block, RichBlock};

impl<Client, Pool, Network> EthApi<Client, Pool, Network>
where
    Client: HeaderProvider + BlockProvider + StateProviderFactory + 'static,
{
    pub(crate) async fn block_by_hash(
        &self,
        hash: H256,
        _full: bool,
    ) -> EthResult<Option<RichBlock>> {
        self.rich_block_from_hash(hash)
    }
    pub(crate) async fn block_by_number(
        &self,
        number: u64,
        _full: bool,
    ) -> EthResult<Option<RichBlock>> {
        // Currently there is no method to get
        // the difficulty directly from a block number,
        // only from a block hash, so the number is first
        // "turned" into its corresponding block hash, and
        // then used to fetch the difficulty.
        let block_id = BlockId::Number(number.into());
        let hash = self.client().block_hash_for_id(block_id)?;
        //Not sure how to handle this unwrap properly.
        self.rich_block_from_hash(hash.unwrap())
    }
    // Given a hash, fetch the matching block and then
    // its total difficulty, but only if they both are found (aka they're both Some(_)).
    fn rich_block_from_hash(&self, hash: H256) -> EthResult<Option<RichBlock>> {
        let client = self.client();
        let block = client.block(BlockId::Hash(hash.0.into()))?;
        let difficulty = client.header_td(&hash)?;
        println!("The block: {:?}", block);
        if let (Some(block), Some(total_difficulty)) = (block, difficulty) {
            // Not quite sure on how to handle this unwrap properly.
            let full_block = Block::from_block_full(block, total_difficulty).unwrap();
            Ok(Some(RichBlock::from(full_block)))
        } else {
            Ok(None)
        }
    }
}
