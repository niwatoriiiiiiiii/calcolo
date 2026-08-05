#![allow(missing_docs)]

use calcolo::lexer::{LexError, tokenize};
use calcolo::token::Token;

#[test]
fn test_tokenize_plus() {
    let input = "3 + 4";
    let result = tokenize(input);

    assert_eq!(
        result,
        Ok(vec![Token::Number(3.0), Token::Plus, Token::Number(4.0),])
    );
}

#[test]
fn test_tokenize_float() {
    let input = "3.14159265";
    let result = tokenize(input);

    assert_eq!(result, Ok(vec![Token::Number(3.14159265),]));
}

#[test]
fn test_tokenize_no_space() {
    let input = "12-3*4";
    let result = tokenize(input);

    assert_eq!(
        result,
        Ok(vec![
            Token::Number(12.0),
            Token::Minus,
            Token::Number(3.0),
            Token::Star,
            Token::Number(4.0),
        ])
    );
}

#[test]
fn test_tokenize_ignores_newlines() {
    let input = "
            3
            *
            4
        ";
    let result = tokenize(input);

    assert_eq!(
        result,
        Ok(vec![Token::Number(3.0), Token::Star, Token::Number(4.0),])
    );
}

#[test]
fn test_tokenize_paren() {
    let input = "(12-3) * 4";
    let result = tokenize(input);

    assert_eq!(
        result,
        Ok(vec![
            Token::LParen,
            Token::Number(12.0),
            Token::Minus,
            Token::Number(3.0),
            Token::RParen,
            Token::Star,
            Token::Number(4.0),
        ])
    );
}

#[test]
fn test_tokenize_numbers_separated_by_newline() {
    let input = "
            3.14
            1
            4.5
        ";
    let result = tokenize(input);

    assert_eq!(
        result,
        Ok(vec![
            Token::Number(3.14),
            Token::Number(1.0),
            Token::Number(4.5),
        ])
    );
}

#[test]
fn test_tokenize_unexpected_char_error() {
    let input = "3 @ 4";
    let result = tokenize(input);

    assert_eq!(result, Err(LexError::UnexpectedChar('@')));
}

#[test]
fn test_tokenize_invalid_number_error() {
    let input = "3.1.4";
    let result = tokenize(input);

    assert_eq!(result, Err(LexError::InvalidNumber("3.1.4".to_string())));

    let input = "3.";
    let result = tokenize(input);

    assert_eq!(result, Err(LexError::InvalidNumber("3.".to_string())));
}
