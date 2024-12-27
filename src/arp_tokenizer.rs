//! This module parses the input into tokens that can then be used for translation

use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete::*;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::{sequence::preceded, IResult};

#[derive(Debug)]
pub enum ArpToken<'a> {
    Ws(&'a str),
    Word(&'a str),
    Template(&'a str),
}

fn parse_template(input: &str) -> IResult<&str, ArpToken> {
    map(preceded(char('&'), alpha1), |s: &str| {
        ArpToken::Template(&s[1..])
    })(input)
}

fn parse_ws(input: &str) -> IResult<&str, ArpToken> {
    map(multispace1, ArpToken::Ws)(input)
}

fn parse_word(input: &str) -> IResult<&str, ArpToken> {
    if input.len() == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            "Nothing to consume",
            ErrorKind::Eof,
        )));
    }
    map(take_while(|c: char| !c.is_whitespace()), ArpToken::Word)(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<ArpToken>> {
    many0(alt((parse_ws, parse_template, parse_word)))(input)
}
