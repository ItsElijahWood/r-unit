/// String Utility from r-unit
///
/// Provides functionality for operations on strings and string slices.
pub mod string;

/// Math Utility from r-unit
/// 
/// Provides general mathematical operators to compare an Any type
pub mod math;

/// Vec Utility from r-unit
///
/// Provides functionality for Vectors
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
