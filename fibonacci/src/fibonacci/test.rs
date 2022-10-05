mod test {
    use crate::fibonacci;
    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(8), 21);
        assert_eq!(fibonacci(28), 317811);
        assert_eq!(fibonacci(44), 701408733);
        assert_eq!(fibonacci(13), 233);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(4), 3);

    }
}