fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    if denominator == 0.0 {
        Err("division by zero")
    } else {
        Ok(numerator / denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10f64, 2f64), Ok(5f64));
        assert_eq!(divide(10f64, 2f64).is_ok(), true);
        assert_eq!(divide(10f64, 2f64).is_err(), false);
        assert_eq!(divide(10f64, 2f64).unwrap(), 5f64);
        assert_eq!(divide(10f64, 0f64), Err("division by zero"));
    }
}
