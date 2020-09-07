// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert() {
        let one = 1;
        let won = 1;
        // assert!(&one != 1); // Cannot compare diff literal types
        // assert!(&1 != 1);
        assert!(&one == &1);
        assert!(one == won);
        assert!(one == 1);
    }
}
