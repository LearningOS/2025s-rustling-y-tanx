// tests1.rs
//
// Tests are important to ensure that your code does what you think it should
// do. Tests can be run on this file with the following command: rustlings run
// tests1
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests1` or use the `hint` watch subcommand for a
// hint.
fn is_even(n: i64) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::is_even;
    #[test]
    fn you_can_assert() {
        assert!(is_even(4));
    }
}
