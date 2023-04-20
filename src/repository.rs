/**

A Least Frequently Used (LFU) Cache implementation in Rust.
This cache stores key-value pairs and evicts the least frequently used ones when the capacity is reached.
It uses a Shortener to generate a unique ID for each value added to the cache.
@param <'a> The lifetime specifier for the cache keys, indicating they can only live as long as the cache itself.
@param <T> The type of values stored in the cache.
@param <N> A constant value indicating the maximum capacity of the cache.
@returns A LFUCache struct instance.
*/
mod shortener;

use std::collections::{HashMap, HashSet, LinkedList};

#[derive(Debug, PartialEq, Clone)]
pub struct Entry<T> {
val: T,
count: usize,
}

impl<T> Entry<T> {
/**
* Increment the access count of this entry.
*/
fn inc(&mut self) {
self.count += 1;
}
}

pub struct LFUCache<'a, T, const N: usize> {
by_key: HashMap<&'a str, Entry<T>>,
freq_node: HashMap<usize, HashSet<&'a str>>,
capacity: usize,
least_freq_node: usize,
shortener: Shortener,
}

impl<'a, T, const N: usize> Default for LFUCache<'a, T, N> {
/**
* Create a new instance of the LFU Cache with the given capacity.
* If the capacity is less than or equal to zero, a panic is thrown.
*/
fn default() -> Self {
if N <= 0 {
panic!("Unable to create cache: capacity is {:?}", N);
}
LFUCache {
by_key: HashMap::new(),
freq_node: HashMap::new(),
capacity: N,
least_freq_node: 0,
shortener: Shortener,
}
}
}

impl<'a, T, const N: usize> LFUCache<'a, T, N> {
/**
* Set a new key-value pair in the cache.
* If the key already exists, its value is updated and its access count is incremented.
* If the cache is at full capacity, the least frequently used entry is evicted.
*
* @param key The key for the new entry.
* @param value The value for the new entry.
*/
pub fn set(&mut self, key: &'a str, value: T) where T: Debug {
if let Some(entry) = self.by_key.get_mut(&key) {
entry.val = self.shortener.next_id(value);
self.update_freq_node(key);
return;
}

    if self.len() >= self.capacity {
        self.evict();
    }

    self.by_key.insert(key, Entry{
        val: value,
        count: 1,
    });

    self.least_freq_node = 1;
    self.freq_node.entry(self.least_freq_node).or_default().insert(key);
}

/**
 * Update the frequency node of a key in the cache.
 * If the key doesn't exist in the cache, this function does nothing.
 * 
 * @param key The key to update.
 */

fn update_freq_node(&mut self, key: &'a str) {
        let entry =  self.by_key.get_mut(key).unwrap();

        // get the freq node
        let freq_node = self.freq_node.get_mut(&entry.count).unwrap();
                    
        // once it have been access
        // we need to remove the key in this current node
        freq_node.remove(key);

        if entry.count == self.least_freq_node && freq_node.is_empty() {
            self.least_freq_node += 1;
        }

        // add to the next freq node
        self.freq_node.entry(entry.count+1).or_default().insert(key);
            
        // increase the access account
        entry.inc();
    }

// return a ref of the most-recently-used entry
    pub fn get(&mut self, key: &'a str) -> Option<&T> where T: Debug{
        self.update_freq_node(key);
        self.by_key.get(key).map(|e| &e.val)
    }

    // evict the least freq used
    fn evict(&mut self) {
        let least_freq_node = self.freq_node.get_mut(&self.least_freq_node).unwrap();
        let least_freq_entry= least_freq_node.pop_front().unwrap();
        self.by_key.remove(least_freq_entry);
    }

   // current cache length
    pub fn len(&self) -> usize {
        self.by_key.len()
    }

    // check if current cache is empty
    pub fn is_empty(&self) -> bool {
        self.by_key.is_empty()
    }

    // current cache capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // clean up the cache
    pub fn clear(&mut self) {
        self.by_key.clear();
        self.freq_node.clear();
    }



