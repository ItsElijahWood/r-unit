
/// Checks if parameter 1 is the same length as parameter 2
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///    let a = "hi";
///    let b = "hi";
///
///    string::matches_length(&a, &b);
/// ```
pub fn matches_length(a: &str, b: &str) {
    if a.len() != b.len() {
        panic!("Parameter 1 does not match the string length as parameter 2")
    }
}
