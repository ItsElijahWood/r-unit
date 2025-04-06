use std::any::Any;

/// Checks if the parameter 1 has an equal type and value to parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::is_equal;
///
/// let a = 5;
/// let b = 5;
/// is_equal::is_equal(&a, &b);
/// ```
pub fn is_equal<A: Any + PartialEq<B>, B: Any>(a: &A, b: &B) {
    if a != b {
        panic!("r-unit catched: Parameter 1 is not equal to parameter 2")
    }
}
