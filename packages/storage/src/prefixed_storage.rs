use cosmwasm_std::Storage;
#[cfg(feature = "iterator")]
use cosmwasm_std::{Order, KV};

use crate::length_prefixed::{to_length_prefixed, to_length_prefixed_nested};
#[cfg(feature = "iterator")]
use crate::namespace_helpers::range_with_prefix;
use crate::namespace_helpers::{get_with_prefix, remove_with_prefix, set_with_prefix};

/// An alias of PrefixedStorage::new for less verbose usage
pub fn prefixed<'a, S>(storage: &'a mut S, namespace: &[u8]) -> PrefixedStorage<'a, S>
where
    S: Storage,
{
    PrefixedStorage::new(storage, namespace)
}

pub struct PrefixedStorage<'a, S>
where
    S: Storage,
{
    storage: &'a mut S,
    prefix: Vec<u8>,
}

impl<'a, S> PrefixedStorage<'a, S>
where
    S: Storage,
{
    pub fn new(storage: &'a mut S, namespace: &[u8]) -> Self {
        PrefixedStorage {
            storage,
            prefix: to_length_prefixed(namespace),
        }
    }

    // Nested namespaces as documented in
    // https://github.com/webmaster128/key-namespacing#nesting
    pub fn multilevel(storage: &'a mut S, namespaces: &[&[u8]]) -> Self {
        PrefixedStorage {
            storage,
            prefix: to_length_prefixed_nested(namespaces),
        }
    }
}

impl<'a, S> Storage for PrefixedStorage<'a, S>
where
    S: Storage,
{
    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        get_with_prefix(self.storage, &self.prefix, key)
    }

    #[cfg(feature = "iterator")]
    /// range allows iteration over a set of keys, either forwards or backwards
    /// uses standard rust range notation, and eg db.range(b"foo"..b"bar") also works reverse
    fn range<'b>(
        &'b self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> Box<dyn Iterator<Item = KV> + 'b> {
        range_with_prefix(self.storage, &self.prefix, start, end, order)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) {
        set_with_prefix(self.storage, &self.prefix, key, value);
    }

    fn remove(&mut self, key: &[u8]) {
        remove_with_prefix(self.storage, &self.prefix, key);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmwasm_std::testing::MockStorage;

    #[test]
    fn multi_level() {
        let mut storage = MockStorage::new();

        // set with nested
        let mut foo = PrefixedStorage::new(&mut storage, b"foo");
        let mut bar = PrefixedStorage::new(&mut foo, b"bar");
        bar.set(b"baz", b"winner");

        // we can nest them the same encoding with one operation
        let loader = PrefixedStorage::multilevel(&mut storage, &[b"foo", b"bar"]);
        assert_eq!(loader.get(b"baz"), Some(b"winner".to_vec()));

        // set with multilevel
        let mut foobar = PrefixedStorage::multilevel(&mut storage, &[b"foo", b"bar"]);
        foobar.set(b"second", b"time");

        let mut a = PrefixedStorage::new(&mut storage, b"foo");
        let b = PrefixedStorage::new(&mut a, b"bar");
        assert_eq!(b.get(b"second"), Some(b"time".to_vec()));
    }
}
