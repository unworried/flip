use self::validator::{assert_ast, ASTNode};

mod expression;
mod validator;

#[test]
fn validation_scheme() {
    let input = "while 1 { \nlet bar = \"TEST\"; \nif 1 == 1 { \nlet foo = 45; \n}; \n};\n";
    let expected = vec![
        ASTNode::While,
        ASTNode::Integer(1),
        ASTNode::Let,
        ASTNode::Ident("bar".to_string()),
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
fn if_statement_binary_condition() {
    let input = "let x = 1; if x == 1 { \nlet foo = \"hello, world!\"; };";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::Integer(1),
        ASTNode::If,
        ASTNode::Binary,
        ASTNode::Ident("x".to_string()),
        ASTNode::Integer(1),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn if_statement_binary_condition_newline() {
    let input = "let x = 1; if x == 1 { \nlet foo = \"hello, world!\"; }; \n";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::Integer(1),
        ASTNode::If,
        ASTNode::Binary,
        ASTNode::Ident("x".to_string()),
        ASTNode::Integer(1),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn if_statement_primary_ident_condition() {
    let input = "let x = 1; if x { \nlet foo = \"hello, world!\"; };";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::Integer(1),
        ASTNode::If,
        ASTNode::Ident("x".to_string()),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn if_statement_primary_ident_condition_newline() {
    let input = "let x = 1; if x { \nlet foo = \"hello, world!\"; }; \n";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::Integer(1),
        ASTNode::If,
        ASTNode::Ident("x".to_string()),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement() {
    let input = "while \"TMP\" { \nlet foo = \"hello, world!\"; };";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement_newline() {
    let input = "while \"TMP\" { \nlet foo = \"hello, world!\"; }; \n";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement_nested_statements() {
    let input = "while \"TMP\" { \nlet x = \"hello, world!\"; \nlet y = \"hello, world 2!\"; \nlet z = \"hello, world 3!\"; \n };";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::String("hello, world!".to_string()),
        ASTNode::Let,
        ASTNode::Ident("y".to_string()),
        ASTNode::String("hello, world 2!".to_string()),
        ASTNode::Let,
        ASTNode::Ident("z".to_string()),
        ASTNode::String("hello, world 3!".to_string()),
    ];

    assert_ast(input, expected)
}

#[test]
fn while_statement_nested_block_statements() {
    let input = "while \"TMP\" { \nlet x = \"hello, world!\";\nif \"TMP\" { \nwhile \"TMP\" { \nlet y = \"hello, world 3!\";\n }; \n }; \n };";

    let expected = vec![
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::String("hello, world!".to_string()),
        ASTNode::If,
        ASTNode::String("TMP".to_string()),
        ASTNode::While,
        ASTNode::String("TMP".to_string()),
        ASTNode::Let,
        ASTNode::Ident("y".to_string()),
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
