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
        let mut a: Vec<String> = Vec::new();
        let string = String::from("h i");

        let might_contain = String::from("hello");

        a.push(string);

        if vec::contains_element(&a, &might_contain) {
            println!("Contains it.");
        } else {
            println!("Does not contain it.");
        }
    }
}
