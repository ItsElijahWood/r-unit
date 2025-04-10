/// Checks if parameter 1 is the same length as parameter 2
///
/// # Example:
/// ```rust
///    use r_unit::string;
///
///    let str1 = "hi";
///    let str2 = "hi";
///
///    assert!(string::matches_length(&str1, &str2));
/// ```
pub fn matches_length(string: &str, string2: &str) -> bool {
    string.len() == string2.len()
}

/// Check if parameter 1 contains special characters or not
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "hi %";
///
///     assert!(string::contains_special_characters(&string));
/// ```
pub fn contains_special_characters(string: &str) -> bool {
    string.chars().any(|c| !c.is_ascii_alphanumeric())
}

/// Check if parameter 1 contains numbers in a string slice or not
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "hi 3";
///
///     assert!(string::contains_numbers(&string));
/// ```
pub fn contains_numbers(string: &str) -> bool {
    string.chars().any(|number| number.is_numeric())
}

/// Check if parameter 1 contains any capital letters in a string slice or not
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "Hi";
///
///     assert!(string::contains_capital_letters(&string));
/// ```
pub fn contains_capital_letters(string: &str) -> bool {
    string.chars().any(|c| c.is_uppercase())
}

/// Check if parameter 1 contains default password requirements
/// # Default requirements:
/// ```text
///     contains at least 1 capital letter
///     contains at least 1 number
///     contains at least 1 special character
///     contains at least 8 characters
/// ```
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let password = "jh+Tr%7f";
///
///     if string::secure_password(&password) {
///         println!("Is a secure password.");
///     } else {
///         println!("Is not a secure password.");
///     };
/// ```
pub fn secure_password(password: &str) -> bool {
    contains_capital_letters(password)
        &&  contains_numbers(password)
        &&  contains_special_characters(password)
        &&  password.len() >= 8
}

/// Check if parameter 1 contains any whitespace
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "h i";
///
///     assert!(string::contains_whitespace(&string));
/// ```
pub fn contains_whitespace(a: &str) -> bool {
    a.chars().any(|char| char.is_whitespace())
}
