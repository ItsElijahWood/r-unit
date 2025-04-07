/// Check if a vector is empty
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let mut vec: Vec<String> = Vec::new();
///     let string = String::from("Hi");
///
///     vec.push(string);
///
///     if vec::is_empty(&vec) {
///         println!("Empty.");
///     } else {
///         println!("Not empty.");
///     };
/// ```
pub fn is_empty<T>(vec: &[T]) -> bool {
    vec.is_empty()
}

/// Check if a vector contains whitespace
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let mut vec: Vec<String> = Vec::new();
///     let string = String::from("H i");
///
///     vec.push(string);
///
///     if vec::has_whitespace(&vec) {
///         println!("Contains whitespace.");
///     } else {
///         println!("Does not contain whitespace.");
///     };
/// ```
pub fn has_whitespace(vec: &Vec<String>) -> bool {
    vec.iter().any(|v| v.chars().any(|s| s.is_whitespace()))
}

/// Check if a vector contains something
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let mut vec: Vec<String> = Vec::new();
///
///     let string = String::from("Hi");
///     let might_contain = String::from("Hi");
///
///     vec.push(string);
///
///     if vec::contains_element(&vec, &might_contain) {
///         println!("Contains {}.", might_contain);
///     } else {
///         println!("Does not contain {}.", might_contain);
///     };
/// ```
pub fn contains_element<T: PartialEq>(vec: &[T], element: &T) -> bool {
    vec.contains(element)
}
