#![allow(missing_docs, dead_code, unused_variables, unused_imports)]
use std::sync::Arc;

use hash256_std_hasher::Hash256StdHasher;
use hash_db::{AsHashDB, Prefix};
use reth_primitives::{keccak256, H256};
use trie_db::{HashDB, Hasher, NodeCodec, TrieDBMut, TrieLayout};

use crate::database::Database;

pub struct DBTrie<'this, DB: Database> {
    db: Arc<DB>,
    trie: TrieDBMut<'this, DBTrieLayout>,
}

impl<'this, DB: Database> DBTrie<'this, DB> {
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

struct HashDatabase;

impl<H: Hasher, T> HashDB<H, T> for HashDatabase {
    fn get(&self, key: &H::Out, prefix: Prefix<'_>) -> Option<T> {
        todo!()
    }

    fn contains(&self, key: &H::Out, prefix: Prefix<'_>) -> bool {
        todo!()
    }

    fn insert(&mut self, prefix: Prefix<'_>, value: &[u8]) -> H::Out {
        todo!()
    }

    fn emplace(&mut self, key: H::Out, prefix: Prefix<'_>, value: T) {
        todo!()
    }

    fn remove(&mut self, key: &H::Out, prefix: Prefix<'_>) {
        todo!()
    }
}

impl<H: Hasher, T> AsHashDB<H, T> for HashDatabase {
    fn as_hash_db(&self) -> &dyn HashDB<H, T> {
        self
    }

    fn as_hash_db_mut<'a>(&'a mut self) -> &'a mut (dyn HashDB<H, T> + 'a) {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::implementation::mdbx::test_utils::create_test_rw_db;
    use reth_libmdbx::WriteMap;
    use reth_primitives::KECCAK_EMPTY;
    use trie_db::TrieDBMutBuilder;

    #[test]
    fn create_trie() {
        let db = create_test_rw_db::<WriteMap>();
        let mut root = KECCAK_EMPTY;
        let mut hash_db = HashDatabase {};
        let builder = TrieDBMutBuilder::new(&mut hash_db, &mut root);
        let trie = DBTrie { db, trie: builder.build() };
    }
}
