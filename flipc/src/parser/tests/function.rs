use std::collections::HashMap;

use super::validator::{assert_program, ASTNode};

#[test]
fn function_no_parameters() {
    let input = r#"main() { x = 4; }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![ASTNode::Variable("x".to_string()), ASTNode::Integer(4)],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_no_parameters_newline() {
    let input = r#"main() { 
        x = 4; 
    }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![ASTNode::Variable("x".to_string()), ASTNode::Integer(4)],
    )]);

    assert_program(input, expected);
}

#[test]
#[should_panic(expected = "diagnostics returned: [\"expected: '}', found: `EoF`\"]")]
fn function_no_parameters_missing_rbrace() {
    let input = r#"main() { x = 4; "#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![ASTNode::Variable("x".to_string()), ASTNode::Integer(4)],
    )]);

    assert_program(input, expected);
}

#[test]
#[should_panic(expected = "diagnostics returned: [\"expected: '}', found: `EoF`\"]")]
fn function_no_parameters_missing_rbrace_newline() {
    let input = r#"main() { 
        x = 4; 
    "#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![ASTNode::Variable("x".to_string()), ASTNode::Integer(4)],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_parameters() {
    let input = r#"main(x, y) { x = 4; }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Variable("y".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_parameters_newline() {
    let input = r#"main(x, y) { 
        x = 4; 
    }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Variable("y".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_trailing_semicolon() {
    let input = r#"main(x, y){x=4;};"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Variable("y".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
#[should_panic(expected = "diagnostics returned: [\"unexpected token: `;`\"]")]
fn function_2_trailing_semicolon() {
    let input = r#"main(x, y){x=4;};;"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Variable("y".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
#[should_panic(expected = "diagnostics returned: [\"unexpected token: `;`\"]")]
fn function_2_trailing_semicolon_newline() {
    let input = r#"main(x, y) { 
        x = 4; 
};;"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Variable("y".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_trailing_semicolon_newline() {
    let input = r#"main(x, y) { 
        x = 4; 
    };"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Variable("y".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
#[should_panic(expected = "diagnostics returned: [\"expected: ',', found: `Ident(y)`\"]")]
fn function_parameters_missing_comma() {
    let input = r#"main(x y) { 
        x = 4; 
    };"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            // No y should be reported as delim(`,`) was not included
            // ASTNode::Variable("y".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call() {
    let input = r#"main() { x = 4; a(); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_variable_arg() {
    let input = r#"main() { x = 4; a(x); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Variable("x".to_string()),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_variable_expr_arg() {
    let input = r#"main() { x = 4; a(x*4); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Binary,
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_multiple_variable_args() {
    let input = r#"main() { x = 4; a(x, x); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Variable("x".to_string()),
            ASTNode::Variable("x".to_string()),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_multiple_variable_expr_args() {
    let input = r#"main() { x = 4; a(-(x/2), 4+x); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Unary,
            ASTNode::Binary,
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(2),
            ASTNode::Binary,
            ASTNode::Integer(4),
            ASTNode::Variable("x".to_string()),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_literal_arg() {
    let input = r#"main() { x = 4; a(210); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Integer(210),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_multiple_literal_args() {
    let input = r#"main() { x = 4; a("Hello World!", -340); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::String("Hello World!".to_string()),
            ASTNode::Unary,
            ASTNode::Integer(340),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_expr_arg() {
    let input = r#"main() { x = 4; a((4+2)*5+7); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Binary,
            ASTNode::Binary,
            ASTNode::Binary,
            ASTNode::Integer(4),
            ASTNode::Integer(2),
            ASTNode::Integer(5),
            ASTNode::Integer(7),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_multiple_expr_args() {
    let input = r#"main() { x = 4; a(-(5/5) * 4, 77 + 1); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Binary,
            ASTNode::Unary,
            ASTNode::Binary,
            ASTNode::Integer(5),
            ASTNode::Integer(5),
            ASTNode::Integer(4),
            ASTNode::Binary,
            ASTNode::Integer(77),
            ASTNode::Integer(1),
        ],
    )]);

    assert_program(input, expected);
}

#[test]
fn function_call_multiple_mixed_args() {
    let input = r#"main() { x = 4; a(-(x/2) * 4/2, "hi there", x - 1*1*1); }"#;
    let expected = HashMap::from([(
        "main".to_string(),
        vec![
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(4),
            ASTNode::Call("a".to_string()),
            ASTNode::Binary,
            ASTNode::Unary,
            ASTNode::Binary,
            ASTNode::Variable("x".to_string()),
            ASTNode::Integer(2),
            ASTNode::Binary,
            ASTNode::Integer(4),
            ASTNode::Integer(2),
            ASTNode::String("hi there".to_string()),
            ASTNode::Binary,
            ASTNode::Variable("x".to_string()),
            ASTNode::Binary,
            ASTNode::Integer(1),
            ASTNode::Binary,
            ASTNode::Integer(1),
            ASTNode::Integer(1),
        ],
    )]);

    assert_program(input, expected);
}
