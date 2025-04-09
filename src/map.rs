use std::{collections::HashMap, hash::Hash};

/// Check if a hashmap is equal to another hashmap
///
/// # Example:
/// ```rust
///     use r_unit::map;
///     
///     let hashmap: HashMap<&str, i32> = HashMap::new();
///     let hashmap2: HashMap<&str, i32> = HashMap::new();
///
///     assert!(map::map_equals(&hashmap, &hashmap2));
/// ```
pub fn map_equals<K, V>(map: &HashMap<K, V>, map2: &HashMap<K, V>) -> bool
where
  K: Eq + Hash,
  V: PartialEq
{
  if map.len() != map2.len() {
    return false;
  } else {
    map.iter().all(|(key, val)| map2.get(key) == Some(val))
  }
}

/// Check if a hashmap is empty
///
/// # Example:
/// ```rust
///     use r_unit::map;
///
///     let hashmap: HashMap<&str, i32> = HashMap::new();
///     assert!(map::is_empty(&hashmap));
/// ```
pub fn is_empty<K, V>(map: &HashMap<K, V>) -> bool {
  map.is_empty()
}
