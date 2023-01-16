#![allow(missing_docs, dead_code, unused_variables, unused_imports)]
use std::sync::Arc;

use hash256_std_hasher::Hash256StdHasher;
use hash_db::{AsHashDB, Prefix};
use reference_trie::ReferenceNodeCodec;
use reth_primitives::{keccak256, H256};
use trie_db::{HashDB, Hasher, NodeCodec, TrieDBMut, TrieLayout};

use crate::database::Database;

pub struct DBTrie<'this, DB: Database> {
    db: Arc<DB>,
    trie: TrieDBMut<'this, DBTrieLayout>,
}

impl<'this, DB: Database> DBTrie<'this, DB> {}

struct DBTrieLayout;

impl TrieLayout for DBTrieLayout {
    const USE_EXTENSION: bool = true;

    // TODO: modify?
    const ALLOW_EMPTY: bool = false;
    // I think non-inlined nodes aren't supported
    const MAX_INLINE_VALUE: Option<u32> = None;

    type Hash = KeccakHasher;
    type Codec = ReferenceNodeCodec<Self::Hash>;
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
