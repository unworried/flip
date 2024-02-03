use crate::ast::tests::validator::ASTNode;

use super::validator::assert_ast;

#[test]
fn identifier() {
    let input = "let test = \"some value\"; if test { let foo = test; };";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("test".to_string()),
        ASTNode::String("some value".to_string()),
        ASTNode::If,
        ASTNode::Ident("test".to_string()),
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::Ident("test".to_string()),
    ];

    assert_ast(input, expected);
}

#[test]
fn literal_int() {
    let input = "let foo = 123;";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::Integer(123),
    ];

    assert_ast(input, expected);
}

#[test]
fn literal_string() {
    let input = "let foo = \"hello, world!\";";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::String("hello, world!".to_string()),
    ];

    assert_ast(input, expected);
}

#[test]
fn unary() {
    let input = "let foo = -123;";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::Unary,
        ASTNode::Integer(123),
    ];

    assert_ast(input, expected);
}

#[test]
fn binary() {
    let input = "let foo = 123 * 456;";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::Binary,
        ASTNode::Integer(123),
        ASTNode::Integer(456),
    ];

    assert_ast(input, expected);
}

#[test]
fn binary_precedence() {
    let input = "let foo = 123 + 456 * 789;";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("foo".to_string()),
        ASTNode::Binary,
        ASTNode::Integer(123),
        ASTNode::Binary,
        ASTNode::Integer(456),
        ASTNode::Integer(789),
    ];

    assert_ast(input, expected);
}

#[test]
fn parenthesis_binary_expression() {
    let input = "let x = (1 + 2);";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::Binary,
        ASTNode::Integer(1),
        ASTNode::Integer(2),
    ];

    assert_ast(input, expected)
}

#[test]
fn binary_expression_nested_parenthesis() {
    let input = "let x = 2*(1 + (2 * 3));";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::Binary,
        ASTNode::Integer(2),
        ASTNode::Binary,
        ASTNode::Integer(1),
        ASTNode::Binary,
        ASTNode::Integer(2),
        ASTNode::Integer(3),
    ];

    assert_ast(input, expected)
}

#[test]
fn nested_parenthesis_binary_expression() {
    let input = "let x = (1 + (2 * 3)) * 4 + 2;";

    let expected = vec![
        ASTNode::Let,
        ASTNode::Ident("x".to_string()),
        ASTNode::Binary,
        ASTNode::Binary,
        ASTNode::Integer(1),
        ASTNode::Binary,
        ASTNode::Integer(2),
        ASTNode::Integer(3),
        ASTNode::Binary,
        ASTNode::Integer(4),
        ASTNode::Integer(2),
    ];

    assert_ast(input, expected)
}

