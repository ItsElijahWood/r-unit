use std::any::Any;

/// Checks if parameter 1 has an equal type and value to parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 5;
/// let b = 5;
///
/// math::is_equal(&a, &b);
/// ```
pub fn is_equal<A: Any + PartialEq<B>, B: Any>(a: &A, b: &B) {
    if a != b {
        panic!("r-unit catched: Parameter 1 is not equal to parameter 2")
    }
}

/// Checks if parameter 1 is not equal type and value to parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 5;
/// let b = 6;
///
/// math::is_not_equal(&a, &b);
/// ```
pub fn is_not_equal<A: Any + PartialEq<B>, B: Any>(a: &A, b: &B) {
    if a == b {
        panic!("r-unit catched: Parameter 1 is equal to parameter 2")
    }
}

/// Checks if parameter 1 is less than parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 5;
/// let b = 6;
///
/// math::is_less_than(&a, &b);
/// ```
pub fn is_less_than(a: &i32, b: &i32) {
    if *a >= *b {
        panic!("r-unit catched: Parameter 1: {} is greater than parameter 2: {}", *a, *b)
    }
}

/// Checks if parameter 1 is greater than parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 7;
/// let b = 6;
///
/// math::is_greater_than(&a, &b);
/// ```
pub fn is_greater_than(a: &i32, b: &i32) {
    if *a <= *b {
        panic!("r-unit catched: Parameter 1: {} is less than parameter 2: {}", *a, *b)
    }
}

/// Checks if parameter 1 is a negative number
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = -3;
///
/// math::is_negative(&a);
/// ```
pub fn is_negative(a: &i32) {
    if *a > 0 {
        panic!("r-unit catched: Parameter 1: {} is not negative", *a)
    }
}

/// Checks if parameter 1 is an odd number
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 7;
///
/// math::is_odd(&a);
/// ```
pub fn is_odd(a: &i32) {
    if a % 2 == 0 {
        panic!("r-unit catched: Parameter 1: {} is not odd", a)
    }
}

/// Checks if parameter 1 is an even number
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 6;
///
/// math::is_even(&a);
/// ```
pub fn is_even(a: &i32) {
    if a % 2 == 1 {
        panic!("r-unit catched: Parameter 1: {} is not even", a)
    }
}
