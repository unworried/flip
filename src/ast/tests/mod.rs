use self::validator::{assert_ast, ASTNode};

mod expression;
mod validator;

#[test]
fn validation_scheme() {
    let input = "while 1 { \nprint \"TEST\"; \nif 1 == 1 { \nlet foo = 45; \n}; \n};\n";
    let expected = vec![
        ASTNode::While,
        ASTNode::Integer(1),
        ASTNode::Print,
        ASTNode::String("TEST".to_string()),
        ASTNode::If,
        ASTNode::Binary,
        ASTNode::Integer(1),
        ASTNode::Integer(1),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::Integer(45),
    ];

    assert_ast(input, expected);
}

#[test]
fn print_string_statement() {
    let input = "print \"hello, world!\";";

    let expected = vec![ASTNode::Print, ASTNode::String("hello, world!".to_string())];

    assert_ast(input, expected)
}

#[test]
fn print_string_statement_newline() {
    let input = "print \"hello, world!\";\n";

    let expected = vec![ASTNode::Print, ASTNode::String("hello, world!".to_string())];

    assert_ast(input, expected)
}

#[test]
fn print_int_statement() {
    let input = "print 123;";

    let expected = vec![ASTNode::Print, ASTNode::Integer(123)];

    assert_ast(input, expected)
}

#[test]
fn print_int_statement_newline() {
    let input = "print 123;\n";

    let expected = vec![ASTNode::Print, ASTNode::Integer(123)];

    assert_ast(input, expected)
}

#[test]
fn if_statement() {
    let input = "if \"TMP\" { \nprint \"hello, world!\"; };";

    let expected = vec![
        ASTNode::If,
        ASTNode::String("TMP".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn if_statement_newline() {
    let input = "if \"TMP\" { \nprint \"hello, world!\"; }; \n";

    let expected = vec![
        ASTNode::If,
        ASTNode::String("TMP".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement() {
    let input = "while \"TMP\" { \nprint \"hello, world!\"; };";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement_newline() {
    let input = "while \"TMP\" { \nprint \"hello, world!\"; }; \n";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement_nested_statements() {
    let input = "while \"TMP\" { \nprint \"hello, world!\"; \nprint \"hello, world 2!\"; \nprint \"hello, world 3!\"; \n };";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world!".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world 2!".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world 3!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement_nested_block_statements() {
    let input = "while \"TMP\" { \nprint \"hello, world!\";\nif \"TMP\" { \nwhile \"TMP\" { \nprint \"hello, world 3!\";\n }; \n }; \n };";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world!".to_string()),
        ASTNode::If,
        ASTNode::String("TMP".to_string()),
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Print,
        ASTNode::String("hello, world 3!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn let_statement() {
    let input = "let Ident = 123;";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("Ident".to_string()),
        ASTNode::Integer(123),
    ];

    assert_ast(input, expected)
}
