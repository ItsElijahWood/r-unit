/// Check if a hashmap is equal to another hashmap
///
/// # Example:
/// ```rust
///     use r_unit::map;
///     use std::collections::HashMap;
///     
///     let hashmap: HashMap<&str, i32> = HashMap::new();
///     let hashmap2: HashMap<&str, i32> = HashMap::new();
///
///     assert!(map::map_equals(&hashmap, &hashmap2));
/// ```
pub fn map_equals<K, V>(map: &std::collections::HashMap<K, V>, map2: &std::collections::HashMap<K, V>) -> bool
where
    K: Eq + std::hash::Hash,
    V: PartialEq,
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
///     use std::collections::HashMap;
///
///     let hashmap: HashMap<&str, i32> = HashMap::new();
///     assert!(map::is_empty(&hashmap));
/// ```
pub fn is_empty<K, V>(map: &std::collections::HashMap<K, V>) -> bool {
    map.is_empty()
}

/// Check if a hashmap contains an element
///
/// # Example:
/// ```rust
///     use r_unit::map;
///     use std::collections::HashMap;
///
///     let mut hashmap: HashMap<&str, i32> = HashMap::new();
///     hashmap.insert("Hi", 5);
///
///     let element = "Hi";
///     assert!(map::contains_element(&hashmap, &element));
/// ```
pub fn contains_element<K, V>(map: &std::collections::HashMap<K, V>, element: &str) -> bool 
where 
    K: std::hash::Hash + Eq + std::borrow::Borrow<str>,
{
    map.contains_key(element)
}

