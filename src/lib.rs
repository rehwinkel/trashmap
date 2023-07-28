use std::{
    collections::{hash_map::DefaultHasher, LinkedList},
    hash::{Hash, Hasher},
};

const TRASH_MAP_START_SIZE: usize = 3;
const TRASH_MAP_LOAD_FACTOR_THRESH: f32 = 0.75;

#[derive(Clone, Debug)]
struct Bucket<K, V> {
    chain: LinkedList<(K, V)>,
}

impl<K: Eq + PartialEq, V> Bucket<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if self.chain.is_empty() {
            self.chain.push_back((key, value));
        } else {
            for element in self.chain.iter_mut() {
                // entry is identical to existing entry
                if element.0.eq(&key) {
                    element.1 = value;
                    return;
                }
            }
            self.chain.push_front((key, value));
        }
    }

    fn get(&self, key: &K) -> Option<&V> {
        if self.chain.len() == 0 {
            None
        } else {
            for element in self.chain.iter() {
                if element.0.eq(key) {
                    return Some(&element.1);
                }
            }
            None
        }
    }

    fn remove(&mut self, key: &K) -> bool {
        if self.chain.len() == 0 {
            return false;
        } else {
            for (i, element) in self.chain.iter().enumerate() {
                if element.0.eq(key) {
                    let mut tail = self.chain.split_off(i);
                    tail.pop_front();
                    self.chain.append(&mut tail);
                    return true;
                }
            }
            false
        }
    }
}

#[derive(Debug)]
pub struct TrashMap<K, V> {
    buckets: Vec<Bucket<K, V>>,
    elements: usize,
}

fn is_prime(number: usize) -> bool {
    if number & 1 == 0 {
        return false;
    }
    let closest_sqrt_integer = (number as f32).sqrt().ceil() as usize;
    for i in 3..closest_sqrt_integer {
        if number % i == 0 {
            return false;
        }
    }
    true
}

fn find_next_prime(prime: usize) -> usize {
    let mut candidate = prime;
    while !is_prime(candidate) {
        candidate += 2;
    }
    return candidate;
}

impl<K: Hash + Eq + PartialEq, V> TrashMap<K, V> {
    fn make_buckets(count: usize) -> Vec<Bucket<K, V>> {
        let mut buckets = Vec::with_capacity(count);
        for _ in 0..count {
            buckets.push(Bucket {
                chain: LinkedList::new(),
            });
        }
        buckets
    }

    pub fn new() -> Self {
        TrashMap {
            buckets: TrashMap::make_buckets(TRASH_MAP_START_SIZE),
            elements: 0,
        }
    }

    fn hash(len: usize, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % (len as u64)
    }

    fn compute_load_factor(&self) -> f32 {
        self.elements as f32 / self.buckets.len() as f32
    }

    fn grow(&mut self) {
        let new_size = find_next_prime(self.buckets.len() * 2 + 1);
        let new_buckets: Vec<Bucket<K, V>> = TrashMap::make_buckets(new_size);
        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
        for (key, value) in old_buckets.into_iter().flat_map(|b| b.chain.into_iter()) {
            TrashMap::insert_into_buckets(&mut self.buckets, key, value);
        }
    }

    fn insert_into_buckets(buckets: &mut Vec<Bucket<K, V>>, key: K, value: V) {
        let hash = TrashMap::<K, V>::hash(buckets.len(), &key);
        let bucket = &mut buckets[hash as usize];
        bucket.insert(key, value);
    }

    pub fn insert(&mut self, key: K, value: V) {
        TrashMap::insert_into_buckets(&mut self.buckets, key, value);
        self.elements += 1;
        if self.compute_load_factor() > TRASH_MAP_LOAD_FACTOR_THRESH {
            self.grow();
        }
    }

    pub fn remove(&mut self, key: &K) -> bool {
        let hash = TrashMap::<K, V>::hash(self.buckets.len(), &key);
        let bucket = &mut self.buckets[hash as usize];
        let removed = bucket.remove(key);
        if removed {
            self.elements -= 1;
        }
        removed
    }

    pub fn len(&self) -> usize {
        self.elements
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = TrashMap::<K, V>::hash(self.buckets.len(), &key);
        let bucket = &self.buckets[hash as usize];
        bucket.get(key)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets
            .iter()
            .flat_map(|b| b.chain.iter())
            .map(|e| (&e.0, &e.1))
    }
}

#[cfg(test)]
mod tests;
