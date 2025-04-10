/// Provides functionality for hashmaps
pub mod map;
/// Provides general mathematical operators to compare types implementing Any
pub mod math;
/// Provides functionality for operations on String and &str.
pub mod string;
/// Provides functionality for &Vec<T> and an array of any type
pub mod vec;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_exec() {
        let mut map = HashMap::new();
        map.insert("Hi!", 5);

        let str = "Hi!";
        assert!(map::contains_element(&map, &str));
    }
}
