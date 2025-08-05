//! In-Memory Cache Example

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type Cache<K, V> = Arc<RwLock<HashMap<K, V>>>;

pub fn new_cache<K: std::cmp::Eq + std::hash::Hash, V>() -> Cache<K, V> {
    Arc::new(RwLock::new(HashMap::new()))
}
