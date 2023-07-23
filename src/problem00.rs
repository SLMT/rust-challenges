/// Example try-out function
pub fn hello() -> &'static str {
    "world"
}

/// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!("world", hello());
    }
}
