//! Minimal S-expression parser (~30 lines of core logic).
//!
//! Parses strings like:
//! ```lisp
//! (package "ripgrep" "14.1.0"
//!   (deps)
//!   (build (extract tar-gz))
//!   (install (to-bin "rg")))
//! ```

use crate::ast::Expr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unexpected end of input")]
    UnexpectedEof,
    #[error("unexpected character: {0}")]
    UnexpectedChar(char),
    #[error("unclosed string")]
    UnclosedString,
    #[error("unclosed list")]
    UnclosedList,
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    let mut chars = input.chars().peekable();
    parse_expr(&mut chars)
}

fn parse_expr(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Expr, ParseError> {
    skip_whitespace_and_comments(chars);

    match chars.peek() {
        None => Err(ParseError::UnexpectedEof),
        Some('(') => parse_list(chars),
        Some('"') => parse_string(chars),
        Some(_) => parse_atom(chars),
    }
}

fn parse_list(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Expr, ParseError> {
    chars.next(); // consume '('
    let mut items = Vec::new();

    loop {
        skip_whitespace_and_comments(chars);
        match chars.peek() {
            None => return Err(ParseError::UnclosedList),
            Some(')') => {
                chars.next();
                return Ok(Expr::List(items));
            }
            Some(_) => items.push(parse_expr(chars)?),
        }
    }
}

fn parse_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Expr, ParseError> {
    chars.next(); // consume opening '"'
    let mut s = String::new();

    loop {
        match chars.next() {
            None => return Err(ParseError::UnclosedString),
            Some('"') => return Ok(Expr::Atom(s)),
            Some('\\') => {
                // Handle escape sequences
                match chars.next() {
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    Some(c) => {
                        s.push('\\');
                        s.push(c);
                    }
                    None => return Err(ParseError::UnclosedString),
                }
            }
            Some(c) => s.push(c),
        }
    }
}

fn parse_atom(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Expr, ParseError> {
    let mut s = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() || c == '(' || c == ')' || c == '"' || c == ';' {
            break;
        }
        s.push(chars.next().unwrap());
    }

    if s.is_empty() {
        Err(ParseError::UnexpectedChar(chars.peek().copied().unwrap_or(' ')))
    } else {
        Ok(Expr::Atom(s))
    }
}

fn skip_whitespace_and_comments(chars: &mut std::iter::Peekable<std::str::Chars>) {
    loop {
        // Skip whitespace
        while chars.peek().map_or(false, |c| c.is_whitespace()) {
            chars.next();
        }
        // Skip line comments starting with ';'
        if chars.peek() == Some(&';') {
            while chars.peek().map_or(false, |&c| c != '\n') {
                chars.next();
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        assert_eq!(parse("foo").unwrap(), Expr::Atom("foo".into()));
    }

    #[test]
    fn test_string() {
        assert_eq!(parse(r#""hello""#).unwrap(), Expr::Atom("hello".into()));
    }

    #[test]
    fn test_string_with_escape() {
        assert_eq!(parse(r#""hello\nworld""#).unwrap(), Expr::Atom("hello\nworld".into()));
    }

    #[test]
    fn test_empty_list() {
        assert_eq!(parse("()").unwrap(), Expr::List(vec![]));
    }

    #[test]
    fn test_simple_list() {
        assert_eq!(
            parse("(foo bar)").unwrap(),
            Expr::List(vec![Expr::Atom("foo".into()), Expr::Atom("bar".into())])
        );
    }

    #[test]
    fn test_nested_list() {
        assert_eq!(
            parse("(foo (bar baz))").unwrap(),
            Expr::List(vec![
                Expr::Atom("foo".into()),
                Expr::List(vec![Expr::Atom("bar".into()), Expr::Atom("baz".into())])
            ])
        );
    }

    #[test]
    fn test_comment() {
        assert_eq!(
            parse("; comment\n(foo)").unwrap(),
            Expr::List(vec![Expr::Atom("foo".into())])
        );
    }

    #[test]
    fn test_package() {
        let input = r#"
            (package "ripgrep" "14.1.0"
              (deps)
              (build (extract tar-gz)))
        "#;
        let expr = parse(input).unwrap();
        assert!(matches!(expr, Expr::List(_)));
    }
}
