mod tests
{
    use std::env;

    #[test]
    fn test_arguments()
    {
        let args: Vec<String> = env::args().skip(1).collect();

        let is_valid: bool = args.len() >= 1;
        assert_eq!(is_valid, true);
    }
}
