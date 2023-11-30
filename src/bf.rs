use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::Hash;

pub struct BloomFilter<'a, V: Hash> {
    size: usize,
    bitset: Cow<'a, [u8]>,
    hashers: Vec<Box<dyn Fn(V) -> u64>>,
}

impl<'a, K: Hash + Clone + Debug> BloomFilter<'a, K> {
    pub fn new(size: usize, hashers: Vec<Box<dyn Fn(K) -> u64>>) -> Self {
        let bitset_size = size.div_ceil(8);
        let bitset = Cow::Owned(vec![0_u8; bitset_size]);
        BloomFilter {
            size,
            bitset,
            hashers,
        }
    }

    pub fn insert(mut self, k: K) -> Self {
        println!("{:?}", self.bitset);
        for hasher in self.hashers.iter_mut() {
            let h = hasher(k.clone());
            let pos = h % self.size as u64;
            let bit = pos / 8;
            let mask = 1 << (7 - (pos % 8));
            println!("(insert) {:?}: {}/{}", k, bit, mask);
            self.bitset.to_mut()[bit as usize] = self.bitset[bit as usize] | mask;
        }
        println!("{:?}", self.bitset);
        self
    }

    pub fn contains(&mut self, k: K) -> bool {
        for hasher in self.hashers.iter_mut() {
            let h = hasher(k.clone());
            let pos = h % self.size as u64;
            let bit = pos / 8;
            let mask = 1 << (7 - (pos % 8));
            println!("{:?}: {}/{}", k, bit, mask);
            if self.bitset[bit as usize] & mask == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher};

    use super::BloomFilter;

    #[test]
    fn test_bf() {
        let hasher = |v: &str| {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            v.hash(&mut hasher);
            hasher.finish()
        };

        let mut bf = BloomFilter::new(100, vec![Box::new(hasher)]);
        bf = bf.insert("hello");
        bf = bf.insert("world");
        println!("{}", bf.contains("hello"));
        println!("{}", bf.contains("world"));
        println!("{}", bf.contains("foo"));
        println!("{}", bf.contains("hell"));
        println!("{}", bf.contains("helloo"));
    }
}
