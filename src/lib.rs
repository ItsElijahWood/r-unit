/// Provides functionality for operations on String and &str.
pub mod string;
/// Provides general mathematical operators to compare types implementing Any
pub mod math;
/// Provides functionality for &Vec<T> and an array of any type
pub mod vec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let vector = vec![2, 5, 100, 100];

        if let Some(min) = vec::max_element(&vector) {
            println!("{}", min);
        };
    }
}
