pub fn if_statement(number: i32) -> &'static str {
    if number < 5 {
        "smaller"
    } else {
        "bigger"
    }
}

pub fn if_else_expression() -> &'static str {
    let number = 3;
    let message = if number < 5 {
        "smaller than 5"
    } else {
        "greater than or equal to 5"
    };
    message
}
