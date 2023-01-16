#![allow(missing_docs, dead_code, unused_variables, unused_imports)]
use std::{cell::RefCell, marker::PhantomData, sync::Arc};

use hash256_std_hasher::Hash256StdHasher;
use hash_db::{AsHashDB, Prefix};
use reference_trie::ReferenceNodeCodec;
use reth_primitives::{keccak256, H256, KECCAK_EMPTY};
use trie_db::{
    CError, HashDB, Hasher, NodeCodec, TrieDBMut, TrieDBMutBuilder, TrieLayout, TrieMut,
};

use crate::{
    database::Database,
    table::{Decode, Encode, Table},
};

pub struct DBTrie<'this, DB, T>
where
    DB: Database,
    T: Table,
    T::Key: Encode + Decode,
{
    trie: TrieDBMut<'this, DBTrieLayout>,
    _t: PhantomData<(DB, T)>,
}

impl<'this, 'db, DB: Database + 'this, T: Table> DBTrie<'this, DB, T>
where
    DB: Database,
    T: Table,
    T::Key: Encode + Decode,
    T::Value: From<Vec<u8>>,
{
    pub fn new(hash_db: &'this mut HashDatabase<DB>, root: &'this mut H256) -> Self {
        let builder = TrieDBMutBuilder::new(hash_db, root);
        Self { trie: builder.build(), _t: Default::default() }
    }

    pub fn get(self, key: T::Key) -> Result<Option<T::Value>, TrieError> {
        let value = self.trie.get(key.encode().as_ref())?;
        Ok(value.map(|v| T::Value::from(v)))
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum TrieError {
    #[error("{0:?}")]
    ImplError(#[from] Box<trie_db::TrieError<reth_primitives::H256, parity_scale_codec::Error>>),
    #[error("{0:?}")]
    DecodeError(#[from] crate::Error),
}

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

pub struct HashDatabase<DB: Database> {
    db: Arc<DB>,
}

impl<H: Hasher, DB: Database, T> HashDB<H, T> for HashDatabase<DB> {
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

impl<H: Hasher, T, DB: Database> AsHashDB<H, T> for HashDatabase<DB> {
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
    use crate::{implementation::mdbx::test_utils::create_test_rw_db, tables};
    use reth_libmdbx::WriteMap;
    use reth_primitives::{hex_literal::hex, Address, KECCAK_EMPTY};
    use trie_db::TrieDBMutBuilder;

    #[test]
    fn create_trie() {
        let db = create_test_rw_db::<WriteMap>();
        let mut root = KECCAK_EMPTY;
        let mut hash_db = HashDatabase { db };
        let trie: DBTrie<'_, _, tables::PlainStorageState> = DBTrie::new(&mut hash_db, &mut root);
        assert_eq!(trie.get(root).unwrap(), None);
    }
}
