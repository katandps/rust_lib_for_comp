//! # キーの集合に関数を適用できるHashMap

use crate::prelude::*;

pub struct ExtendMap<K, V, H = RandomState> {
    map: HashMap<K, V, H>,
    offset: K,
}

impl<K: Clone, V: Clone> Clone for ExtendMap<K, V, RandomState> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            offset: self.offset.clone(),
        }
    }
}

impl<K: Default + Eq + Hash, V: Default> Default for ExtendMap<K, V, RandomState> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            offset: Default::default(),
        }
    }
}

impl<K: Eq + Hash + Add<Output = K> + Sub<Output = K> + Copy, V> ExtendMap<K, V, RandomState> {
    /// # entry interface
    pub fn entry(&mut self, key: K) -> std::collections::hash_map::Entry<K, V> {
        self.map.entry(key)
    }

    /// # 挿入
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    /// # 取得
    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(&(*key - self.offset))
    }

    /// # すべてのキーにxを加算する
    pub fn add_offset(&mut self, offset: K) {
        self.offset = self.offset + offset
    }
}
impl<K: Eq + Display + Debug + Hash + Add<Output = K> + Sub<Output = K> + Copy, V: Debug> Debug
    for ExtendMap<K, V, RandomState>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.map
                .iter()
                .map(|(k, v)| (*k + self.offset, v))
                .collect::<HashMap<_, _>>()
        )
    }
}

#[test]
fn test() {
    let mut exmap = ExtendMap::default();
    *exmap.entry(123).or_insert(0) += 5;
    assert_eq!(Some(&5), exmap.get(&123));
    exmap.add_offset(12);
    assert_eq!(Some(&5), exmap.get(&135));
}
