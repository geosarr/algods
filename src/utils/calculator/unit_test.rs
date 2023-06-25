#[cfg(test)]
mod tests {
    use super::super::Calculator;
    #[test]
    fn test_calculator() {
        let mut e = Calculator::new();
        let exp = "( ( 1 * ( 2 + 3 ) ) + ( ( 4 * ( 5 + 6 ) ) + ( 10 : ( 4 + 1 ) ) ) )".to_string();
        let res = e.compute(exp);
        assert_eq!(res, 51);
    }
}
