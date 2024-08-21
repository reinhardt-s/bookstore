pub mod string_utils {
    /// Checks if a string is empty (after trimming whitespace).
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check.
    ///
    /// # Returns
    ///
    /// * `true` if the string is empty or contains only whitespace, `false` otherwise.
    pub fn is_empty_string(s: &str) -> bool {
        s.trim().is_empty()
    }
}

pub mod math_utils {
    /// Checks if a number is even.
    ///
    /// # Arguments
    ///
    /// * `num` - The number to check.
    ///
    /// # Returns
    ///
    /// * `true` if the number is even, `false` otherwise.
    pub fn is_even(num: i32) -> bool {
        num % 2 == 0
    }

    /// Checks if a number is odd.
    ///
    /// # Arguments
    ///
    /// * `num` - The number to check.
    ///
    /// # Returns
    ///
    /// * `true` if the number is odd, `false` otherwise.
    pub fn is_odd(num: i32) -> bool {
        num % 2 != 0
    }

    /// Calculates the factorial of a number.
    ///
    /// # Arguments
    ///
    /// * `num` - The number for which to calculate the factorial.
    ///
    /// # Returns
    ///
    /// * The factorial of the number.
    pub fn factorial(num: u32) -> u32 {
        match num {
            0 => 1,
            _ => num * factorial(num - 1),
        }
    }
}
