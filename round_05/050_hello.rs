fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10f64, 2f64), Some(5f64));
        assert_eq!(divide(10f64, 0f64), None);
        assert_eq!(10f64 / 0f64, std::f64::INFINITY);
    }
}
