use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

// TODO replace by https://crates.io/crates/bimap
pub struct DoubleLookupTables<K, V> {
    lookup_k: HashMap<K, V>,
    lookup_v: HashMap<V, K>,
}

impl<K, V> DoubleLookupTables<K, V>
where
    K: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            lookup_k: HashMap::new(),
            lookup_v: HashMap::new(),
        }
    }

    pub fn get_by_key(&self, k: &K) -> Option<&V> {
        self.lookup_k.get(k)
    }

    pub fn get_by_value(&self, v: &V) -> Option<&K> {
        self.lookup_v.get(v)
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.lookup_k.insert(k.clone(), v.clone());
        self.lookup_v.insert(v, k);
    }
}

impl<K, V> Default for DoubleLookupTables<K, V>
where
    K: Hash + Eq + Clone,
    V: Hash + Eq + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_lookup_table() {
        let mut dlt = DoubleLookupTables::<u8, char>::new();
        dlt.insert(97, 'a');
        dlt.insert(100, 'd');
        assert_eq!(dlt.get_by_value(&'z'), None);
        assert_eq!(dlt.get_by_value(&'d'), Some(&100));
        assert_eq!(dlt.get_by_key(&97), Some(&'a'));
        assert_eq!(dlt.get_by_key(&5), None);
    }
}
