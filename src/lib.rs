/// String Utility from r-unit
///
/// Provides functionality for operations on strings and string slices.
pub mod string;

/// Math Utility from r-unit
/// 
/// Provides general mathematical operators to compare an Any type
pub mod math;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let a = "hi";

        if string::contains_numbers(&a) {
            println!("Contains numbers");
        } else {
            println!("Does not contain numbers");
        };
    }
}
