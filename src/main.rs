mod basics;
mod loops;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sum_number() {
        let sum: i8 = basics::variables::let_sum_number(12, 5);
        assert_eq!(sum, 17);
    }

    #[test]
    fn it_test_welcome() {
        assert_eq!(basics::variables::greeting(), "I'm ready to learn Rust!");
        assert_eq!(basics::variables::greet("Mi"), "Hello, Mi!");
        assert_eq!(11, 1 + 2 * 5);
    }

    #[test]
    fn control_flow() {
        let res = basics::control_flow::if_statement(3);
        assert_eq!(res, "smaller");
        let res = basics::control_flow::if_statement(5);
        assert_eq!(res, "bigger");
        let res = basics::control_flow::if_else_expression();
        assert_eq!(res, "smaller than 5");
    }
}

fn main() {
    // Basic DT
    // basics::main();
    loops::main()


}
