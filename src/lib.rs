//! This crate entroduces [`is_true!`] macro and [`is_true()`] function which checks if argument is true

/// Macro checks if all provided expressions are true
/// ## Examples
/// ```rust
/// use is_true::is_true;
///
/// fn main() {
///     let result = is_true!(4 % 2 == 0, 5 % 2 == 0);
///     assert_eq!(result, false);        
/// }
/// ```
/// ```rust
/// use is_true::is_true;
///
/// fn main() {
///     assert_eq!(is_true!(true), true);
/// }
/// ```

#[macro_export]
macro_rules! is_true {
    ( $( $x:expr ),* ) => {
        {
            let mut result = true;
            $(
                result &= $x;
            )*
            result
        }
    };
}

/// Function checks if argument is true
#[inline]
pub fn is_true(arg: bool) -> bool {
    arg
}

#[cfg(test)]
mod tests {
    use crate::is_true;

    #[test]
    fn yes_its_true() {
        assert_eq!(is_true(true), true);
        assert_eq!(is_true!(true), true);
    }
    #[test]
    fn no_its_false() {
        assert_eq!(is_true(false), false);
        assert_eq!(is_true!(false), false);
    }
    #[test]
    fn multiple() {
        assert_eq!(is_true!(true, true, false), false);
        assert_eq!(is_true!(true, true, true), true);
    }
}
