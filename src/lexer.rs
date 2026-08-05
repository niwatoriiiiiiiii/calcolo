use crate::token::Token;

/// Errors that can occur during lexical analysis.
#[derive(Debug, PartialEq)]
pub enum LexError {
    /// An unknown character was encountered.
    UnexpectedChar(char),
    /// An invalid number literal was encountered (e.g., "3.", "3.1.4").
    InvalidNumber(String),
}

/// Converts the input string into a sequence of tokens.
///
/// # Errors
///
/// Returns an error if an unknown character is encountered.
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_whitespace() {
            chars.next();
            continue;
        }

        match c {
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            '0'..='9' => {
                let number = read_number(&mut chars)?;
                tokens.push(Token::Number(number));
            }

            _ => {
                return Err(LexError::UnexpectedChar(c));
            }
        }
    }

    Ok(tokens)
}

fn read_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<f64, LexError> {
    let mut num_str = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            num_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    // Block it because parsing succeeds if it ends with a dot.
    if num_str.ends_with('.') {
        return Err(LexError::InvalidNumber(num_str));
    }

    num_str
        .parse::<f64>()
        .map_err(|_| LexError::InvalidNumber(num_str))
}
