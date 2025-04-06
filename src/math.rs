use std::any::Any;

/// Checks if the parameter 1 has an equal type and value to parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 5;
/// let b = 5;
/// math::is_equal(&a, &b);
/// ```
pub fn is_equal<A: Any + PartialEq<B>, B: Any>(a: &A, b: &B) {
    if a != b {
        panic!("r-unit catched: Parameter 1 is not equal to parameter 2")
    }
}

/// Checks if the parameter 1 is not equal type and value to parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 5;
/// let b = 6;
/// math::is_not_equal(&a, &b);
/// ```
pub fn is_not_equal<A: Any + PartialEq<B>, B: Any>(a: &A, b: &B) {
    if a == b {
        panic!("r-unit catched: Parameter 1 is equal to parameter 2")
    }
}

