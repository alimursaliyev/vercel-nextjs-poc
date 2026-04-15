// dummy crate to satisfy path filter
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// POC change to trigger workflow path filter
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
