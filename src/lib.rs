/// Provides functionality for operations on String and &str.
pub mod string;
/// Provides general mathematical operators to compare types implementing Any
pub mod math;
/// Provides functionality for &Vec<T> and an array of any type
pub mod vec;
/// Provides functionality for hashmaps
pub mod map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let vector = vec![1, 2, 3, 4];

        assert!(vec::is_unique(&vector));
    }
}
