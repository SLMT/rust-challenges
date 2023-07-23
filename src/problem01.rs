fn sum(a: i32, b: i32) -> i32 {
    todo!()
}

fn min(a: i32, b: i32) -> i32 {
    todo!()
}

fn pow(base: i32, power: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(10, sum(2, 8));
        assert_eq!(87, sum(50, 27));
        assert_eq!(0, sum(0, 0));
        assert_eq!(-10, sum(-2, 12));
    }

    #[test]
    fn test_min() {
        assert_eq!(0, min(0, 8));
        assert_eq!(100, min(100, 8787));
        assert_eq!(-10, min(-10, 0));
        assert_eq!(-41, min(-41, -32));
    }

    #[test]
    fn test_power() {
        assert_eq!(9, pow(3, 2));
        assert_eq!(32, pow(2, 5));
        assert_eq!(1, pow(1, 10));
        assert_eq!(0, pow(0, 10));
    }
}
