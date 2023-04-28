// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

// I AM NOT DON

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    /**
    * .product()
    * Iterates over the entire iterator, multiplying all the elements

      An empty iterator returns the one value of the type.
    */
    //(1..=num).product()

    /**
     * fold() takes two arguments: an initial value, and a closure with two arguments:
     * an ‘accumulator’, and an element.
     * The closure returns the value that the accumulator should have for the next iteration.
     */
    //(1..=num).fold(1, |acc, x| acc * x)

    /**
     * rfold() combines elements in a right-associative fashion.
     * For associative operators like +, the order the elements are combined in is not important,
     * but for non-associative operators like - the order will affect the final result.
     */
    (1..=num).rfold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
