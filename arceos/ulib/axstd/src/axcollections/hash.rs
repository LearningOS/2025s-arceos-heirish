use core::hash::{BuildHasher, Hasher};
pub struct DefaultHashBuilder;
pub struct DefaultHasher(u64);
impl DefaultHashBuilder {
    pub fn new() -> Self {
        Self
    }
}

/*pub struct DefaultHashBuilder;
| ----------------------------- doesn't satisfy `DefaultHashBuilder: BuildHasher`
|
= note: the following trait bounds were not satisfied:
        `K: core::cmp::Eq`
        `K: Hash`
        `DefaultHashBuilder: BuildHasher`
note: the trait `BuildHasher` must be implemented
*/
impl BuildHasher for DefaultHashBuilder {
    type Hasher = DefaultHasher;
    fn build_hasher(&self) -> Self::Hasher {
        DefaultHasher(arceos_api::sys::ax_random() as u64)
    }
}
/*
error[E0277]: the trait bound `DefaultHasher: Hasher` is not satisfied
   --> ulib/axstd/src/axcollections/hash.rs:20:19
    |
20  |     type Hasher = DefaultHasher;
    |                   ^^^^^^^^^^^^^ the trait `Hasher` is not implemented for `DefaultHasher`
*/
impl Hasher for DefaultHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = self.0.rotate_left(8).wrapping_add(byte as u64);
        }
    }
}
