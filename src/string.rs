/// Checks if parameter 1 is the same length as parameter 2
///
/// # Example:
/// ```rust
///    use r_unit::string;
///
///    let str1 = "hi";
///    let str2 = "hi";
///
///    if string::matches_length(&str1, &str2) {
///     println!("Matches string length.");
///    } else {
///     println!("Does not match string length.");
///    };
/// ```
pub fn matches_length(a: &str, b: &str) -> bool {
    a.len() == b.len()
}

/// Check if parameter 1 contains special characters or not, returns true or false
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "hi %";
///
///     if string::contains_special_characters(&string) {
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

    false
}

/// Check if parameter 1 contains numbers in a string slice or not, returns true or false
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "hi 3";
///
///     if string::contains_numbers(&string) {
///         println!("Contains special characters.");
///     } else {
///         println!("Does not contain special characters.");
///     };
/// ```
pub fn contains_numbers(a: &str) -> bool {
    a.chars().any(|number| number.is_numeric())
}

/// Check if parameter 1 contains any capital letters in a string slice or not, returns true or false
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "Hi";
///
///     if string::contains_capital_letters(&string) {
///         println!("Contains special characters.");
///     } else {
///         println!("Does not contain special characters.");
///     };
/// ```
pub fn contains_capital_letters(a: &str) -> bool {
    a.chars().any(|c| c.is_uppercase())
}

/// Check if parameter 1 contains default password requirements, returns true or false.
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

/// Check if parameter 1 contains any whitespace, returns true or false
///
/// # Example:
/// ```rust
///     use r_unit::string;
///
///     let string = "h i";
///
///     if string::contains_whitespace(&string) {
///         println!("Contains white space.");
///     } else {
///         println!("Does not contain white space.");
///     };
/// ```
pub fn contains_whitespace(a: &str) -> bool {
    a.chars().any(|char| char.is_whitespace())
}
