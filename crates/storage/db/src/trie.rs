#![allow(missing_docs, dead_code, unused_variables, unused_imports)]
use hash256_std_hasher::Hash256StdHasher;
use reth_primitives::{keccak256, H256};
use trie_db::{Hasher, NodeCodec, TrieDBMut, TrieLayout};

pub struct DBTrie<'this> {
    inner: TrieDBMut<'this, DBTrieLayout>,
}

impl<'this> DBTrie<'this> {
    fn get() {}
}

struct DBTrieLayout;

impl TrieLayout for DBTrieLayout {
    const USE_EXTENSION: bool = true;

    // TODO: modify this two?
    const ALLOW_EMPTY: bool = false;
    const MAX_INLINE_VALUE: Option<u32> = None;

    type Hash = KeccakHasher;
    type Codec = DBCodec;
}

struct KeccakHasher;

impl Hasher for KeccakHasher {
    type Out = H256;

    type StdHasher = Hash256StdHasher;

    const LENGTH: usize = 256 / 8;

    fn hash(x: &[u8]) -> Self::Out {
        keccak256(x)
    }
}

struct DBCodec;

impl NodeCodec for DBCodec {
    type Error = crate::Error;

    type HashOut = H256;

    fn hashed_null_node() -> Self::HashOut {
        todo!()
    }

    fn decode_plan(data: &[u8]) -> Result<trie_db::node::NodePlan, Self::Error> {
        todo!()
    }

    fn is_empty_node(data: &[u8]) -> bool {
        todo!()
    }

    fn empty_node() -> &'static [u8] {
        todo!()
    }

    fn leaf_node(
        partial: impl Iterator<Item = u8>,
        number_nibble: usize,
        value: trie_db::node::Value<'_>,
    ) -> Vec<u8> {
        todo!()
    }

    fn extension_node(
        partial: impl Iterator<Item = u8>,
        number_nibble: usize,
        child_ref: trie_db::ChildReference<Self::HashOut>,
    ) -> Vec<u8> {
        todo!()
    }

    fn branch_node(
        children: impl Iterator<
            Item = impl std::borrow::Borrow<Option<trie_db::ChildReference<Self::HashOut>>>,
        >,
        value: Option<trie_db::node::Value<'_>>,
    ) -> Vec<u8> {
        todo!()
    }

    fn branch_node_nibbled(
        partial: impl Iterator<Item = u8>,
        number_nibble: usize,
        children: impl Iterator<
            Item = impl std::borrow::Borrow<Option<trie_db::ChildReference<Self::HashOut>>>,
        >,
        value: Option<trie_db::node::Value<'_>>,
    ) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_trie() {
        assert!(true);
    }
}
