/// Checks if parameter 1 is the same length as parameter 2
///
/// # Example:
/// ```rust
///    use r_unit::string;
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

/// Check if parameter 1 contains special characters or not, returns true or false
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let a = "hi %";
///
///     if string::contains_special_characters(&a) {
///         println!("Contains special characters.");
///     } else {
///         println!("Does not contain special characters.");
///     };
/// ```
pub fn contains_special_characters(a: &str) -> bool {
    let special_characters = [
        "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "-", "_", "=", "+", "[", "]",
        "{", "}", ";", ":", "'", "\"", "\\", "|", ",", ".", "<", ">", "/", "?", "`", "~"
    ];

    for character in special_characters {
        if a.contains(character) {
            return true
        }
    }

    return false
}
