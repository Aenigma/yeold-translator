//! This module parses the input into tokens that can then be used for translation

use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::*;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::multi::many0;
/// This module provides functionality for tokenizing ARP (Address Resolution Protocol) data.
/// It uses the `nom` crate for parsing sequences of bytes.
///
/// The `preceded` combinator from `nom::sequence` is used to parse input that is preceded by a specific pattern.
/// The `IResult` type is used to represent the result of a parsing operation, which can be either a success or an error.
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum ArpToken<'a> {
    Ws(&'a str),
    Word(&'a str),
}

fn parse_ws(input: &str) -> IResult<&str, ArpToken> {
    map(multispace1, ArpToken::Ws)(input)
}

fn parse_word(input: &str) -> IResult<&str, ArpToken> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            "Nothing to consume",
            ErrorKind::Eof,
        )));
    }
    map(take_while(|c: char| !c.is_whitespace()), ArpToken::Word)(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<ArpToken>> {
    many0(alt((parse_ws, parse_word)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ws() {
        assert_eq!(parse_ws("   "), Ok(("", ArpToken::Ws("   "))));
        assert_eq!(parse_ws("\t"), Ok(("", ArpToken::Ws("\t"))));
        assert_eq!(parse_ws("\n"), Ok(("", ArpToken::Ws("\n"))));
    }

    #[test]
    fn test_parse_word() {
        assert_eq!(parse_word("hello"), Ok(("", ArpToken::Word("hello"))));
        assert_eq!(parse_word("world "), Ok((" ", ArpToken::Word("world"))));
        assert_eq!(parse_word("rust"), Ok(("", ArpToken::Word("rust"))));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("hello world"),
            Ok((
                "",
                vec![
                    ArpToken::Word("hello"),
                    ArpToken::Ws(" "),
                    ArpToken::Word("world")
                ]
            ))
        );
        assert_eq!(
            parse("rust  programming"),
            Ok((
                "",
                vec![
                    ArpToken::Word("rust"),
                    ArpToken::Ws("  "),
                    ArpToken::Word("programming")
                ]
            ))
        );
    }
}
