/// Check if a vector is empty
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let mut vec: Vec<String> = Vec::new();
///     assert!(vec::is_empty(&vec));
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
///     assert!(vec::contains_whitespace(&vec));
/// ```
pub fn contains_whitespace(vec: &Vec<String>) -> bool {
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
///     assert!(vec::contains_element(&vec, &might_contain));
/// ```
pub fn contains_element<T: PartialEq>(vec: &[T], element: &T) -> bool {
    vec.contains(element)
}

/// Check if a vector is in order
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let vector = vec![1, 2, 3, 4];
///
///     assert!(vec::is_sorted(&vector));
/// ```
pub fn is_sorted<T: Ord>(vec: &[T]) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1])
}

/// Check if all elements in a vector are unique
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let vector = vec![1, 2, 3, 4];
///
///     assert!(vec::is_unique(&vector));
/// ```
pub fn is_unique<T>(vec: &[T]) -> bool
where
    T: Eq + std::hash::Hash,
{
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    vec.iter().all(|item| seen.insert(item))
}

/// Find the minimum element in the vector
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let vector = vec![1, 2, 3, 4];
///
///     if let Some(min) = vec::min_element(&vector) {
///         println!("The minimum element in the vector is: {}.", min);
///     } else {
///         println!("The vector is empty.");
///     };
/// ```
pub fn min_element<T: Ord>(vec: &[T]) -> Option<&T> {
    vec.iter().min()
}

/// Find the maximum element in the vector
///
/// # Example:
/// ```rust
///     use r_unit::vec;
///
///     let vector = vec![1, 2, 3, 4];
///
///     if let Some(max) = vec::max_element(&vector) {
///         println!("The maximum element in the vector is: {}.", max);
///     } else {
///         println!("The vector is empty.");
///     };
/// ```
pub fn max_element<T: Ord>(vec: &[T]) -> Option<&T> {
    vec.iter().max()
}
