pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Placeholder for testing boilerplate

#[cfg(test)]
mod tests {
    use super::*; // not correct

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic]
    fn test_add_should_panic() {
        panic!("This test will fail on purpose");
    }
}
