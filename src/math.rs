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
/// assert!(math::is_equal(&a, &b));
/// ```
pub fn is_equal<A: Any + PartialEq<B>, B: Any>(num: &A, num2: &B) -> bool {
    num == num2
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
/// assert!(math::is_not_equal(&a, &b));
/// ```
pub fn is_not_equal<A: Any + PartialEq<B>, B: Any>(num: &A, num2: &B) -> bool {
    num != num2
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
/// assert!(math::is_less_than(&a, &b));
/// ```
pub fn is_less_than(num: &i32, num2: &i32) -> bool {
    *num <= *num2 
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
/// assert!(math::is_greater_than(&a, &b));
/// ```
pub fn is_greater_than(num: &i32, num2: &i32) -> bool {
    *num >= *num2 
}

/// Checks if parameter 1 is a negative number
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = -3;
///
/// assert!(math::is_negative(&a));
/// ```
pub fn is_negative(num: &i32) -> bool {
    *num < 0 
}

/// Checks if parameter 1 is an odd number
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 7;
///
/// assert!(math::is_odd(&a));
/// ```
pub fn is_odd(num: &i32) -> bool {
    num % 2 == 1
}

/// Checks if parameter 1 is an even number
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 6;
///
/// assert!(math::is_even(&a));
/// ```
pub fn is_even(num: &i32) -> bool {
    num % 2 == 0 
}

/// Checks whether parameter 1 is divisible by the divisor specified in parameter 2
///
/// # Examples
/// ```rust
/// use r_unit::math;
///
/// let a = 6;
/// let diviser = 2;
///
/// assert!(math::can_divide(&a, diviser));
/// ```
pub fn can_divide(divided: &i32, diviser: i32) -> bool {
    if diviser == 0 {
        return false;
    }

    divided % diviser == 0
}
