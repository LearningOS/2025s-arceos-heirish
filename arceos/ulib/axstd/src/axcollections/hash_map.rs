use super::DefaultHashBuilder;
use core::hash::{BuildHasher, Hash};
use hashbrown::hash_map as base;
pub struct HashMap<K, V, S = DefaultHashBuilder>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    base: base::HashMap<K, V, S>,
}

pub struct Iter<'a, K: 'a, V: 'a> {
    base: base::Iter<'a, K, V>,
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> HashMap<K, V, DefaultHashBuilder> {
        HashMap {
            base: base::HashMap::with_hasher(DefaultHashBuilder::new()),
        }
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            base: self.base.iter(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
}

/*
 for (k, v) in m.iter() {

   |                   ^^^^^^^^ `std::axcollections::hash_map::Iter<'_, String, u32>` is not an iterator

   |

   = help: the trait `Iterator` is not implemented for `std::axcollections::hash_map::Iter<'_, String, u32>`, which is required by `std::axcollections::hash_map::Iter<'_, String, u32>: IntoIterator`

   = note: required for `std::axcollections::hash_map::Iter<'_, String, u32>` to implement `IntoIterator`
*/
impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.base.next()
    }
}
