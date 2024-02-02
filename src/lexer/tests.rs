use super::*;

fn check_tokens(input: &str, expected: Vec<Token>) {
    let mut lex = Lexer::new(input.to_string());

    for token in expected {
        let next_token = lex.next_token();
        println!("expected: {:?}, got: {:?}", token, next_token);
        assert_eq!(next_token, token);
    }
}

#[test]
fn read_char() {
    let input = "A";
    let mut lex = Lexer::new(input.to_string());

    assert_eq!(lex.ch, b'A');

    lex.read_char();
    assert_eq!(lex.ch, 0);
}

#[test]
fn read_multiple_chars() {
    let input = "let foo = 1";
    let mut lex = Lexer::new(input.to_string());

    for ch in input.chars() {
        assert_eq!(lex.ch, ch as u8);
        lex.read_char();
    }
}

#[test]
fn read_char_empty_input() {
    let input = "";
    let lex = Lexer::new(input.to_string());

    assert_eq!(lex.ch, 0);
}

#[test]
fn peek_char() {
    let input = "AB";
    let lex = Lexer::new(input.to_string());

    assert_eq!(lex.ch, b'A');
    assert_eq!(lex.peek(), b'B');
}

#[test]
fn peek_char_test_pure() {
    let input = "AB";
    let lex = Lexer::new(input.to_string());

    assert_eq!(lex.peek(), b'B');
    assert_eq!(lex.peek(), b'B');
}

#[test]
fn peek_multiple_chars() {
    let input = "let bar = 55";
    let mut lex = Lexer::new(input.to_string());

    while lex.peek() != 0 {
        let next_char = lex.peek();
        lex.read_char();
        assert_eq!(lex.ch, next_char);
    }
}

#[test]
fn peek_char_eof() {
    let input = "A";
    let lex = Lexer::new(input.to_string());

    assert_eq!(lex.ch, b'A');
    assert_eq!(lex.peek(), EOF);
}

#[test]
fn tokenize_whitespace() {
    let input = " \t\n\r";
    let mut lex = Lexer::new(input.to_string());

    let next_token = lex.next_token();
    assert_eq!(next_token, Token::Newline);
    let next_token = lex.next_token();
    assert_eq!(next_token, Token::Eof);
}

#[test]
fn tokenize_arithmetic_operations() {
    let input = "+-*/";

    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::ForwardSlash,
    ];

    check_tokens(input, expected);
}

#[test]
fn tokenize_with_whitespace() {
    let input = "+- */";

    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::ForwardSlash,
    ];

    check_tokens(input, expected);
}

#[test]
fn tokenize_operations() {
    let input = "+- */ >>= = !=";

    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::Asterisk,
        Token::ForwardSlash,
        Token::GreaterThan,
        Token::GreaterThanEqual,
        Token::Assign,
        Token::NotEqual,
    ];

    check_tokens(input, expected);
}

#[test]
fn tokenize_comment() {
    let input = "# this is a comment";
    let mut lex = Lexer::new(input.to_string());

    let next_token = lex.next_token();
    assert_eq!(next_token, Token::Eof);
}

#[test]
fn tokenize_comment_with_newline() {
    let input = "# this is a comment\n";
    let mut lex = Lexer::new(input.to_string());

    let next_token = lex.next_token();
    assert_eq!(next_token, Token::Newline);
    let next_token = lex.next_token();
    assert_eq!(next_token, Token::Eof);
}

#[test]
fn tokenize_with_comment() {
    let input = "+- # comment here == <= >= != = -- , ;\n */";

    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::Newline,
        Token::Asterisk,
        Token::ForwardSlash,
    ];

    check_tokens(input, expected);
}

#[test]
fn tokenize_string() {
    let input = "+- \"string12345\" # comment \n */";

    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::String(String::from("string12345")),
        Token::Newline,
        Token::Asterisk,
        Token::ForwardSlash,
    ];

    check_tokens(input, expected);
}

#[test]
fn tokenize_int() {
    let input = "+-123 98654#comment\n*/";

    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::Int(String::from("123")),
        Token::Int(String::from("98654")),
        Token::Newline,
        Token::Asterisk,
        Token::ForwardSlash,
    ];

    check_tokens(input, expected);
}

#[test]
fn tokenize_complete() {
    let input = "if+-123 foo*then \n/98654#comment\n*/";

    let expected = vec![
        Token::If,
        Token::Plus,
        Token::Minus,
        Token::Int(String::from("123")),
        Token::Ident(String::from("foo")),
        Token::Asterisk,
        Token::Then,
        Token::Newline,
        Token::ForwardSlash,
        Token::Int(String::from("98654")),
        Token::Newline,
        Token::Asterisk,
        Token::ForwardSlash,
    ];

    check_tokens(input, expected)
}
