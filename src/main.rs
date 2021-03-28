use std::ops;
use core::ops::Range;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while_m_n;
use nom::InputLength;
use nom::InputTake;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Integer(u64),
    #[regex("[a-zA-z]+")]
    String_,
    #[token("=")]
    Equals,
    #[token("+")]
    Plus,
    #[regex(r"[ \t\f]+", logos::skip)]
    #[error]
    Error
}

impl Token {
    fn split_at(&self, count: usize) -> &[Token] {

    }
}

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        0
    }
}

impl ops::Index<usize> for Token {
    type Output = Token;
    fn index(&self, index: usize) -> &Token {
        &(*self)[index]
    }
}


impl ops::Index<Range<usize>> for Token {
    type Output = Token;
    fn index(&self, index: Range<usize>) -> &Token {
        &(*self)[index]
    }
}

impl InputTake for Token {
    #[inline]
    fn take(&self, count: usize) -> Self {
        self[0..1]
    }
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.split_at(count);
        (suffix, prefix)
    }
}

fn parse_string(i: &[Token]) -> IResult<Token, Token> {
    let (to, _) = tag(Token::Equals)(i)?;
    to
}


fn hex_color(i: &str) -> IResult<&str, &str> {
    let (tg, _) = tag("#")(i)?; // get first char 
    take_while_m_n(6, 6, |c: char| c.is_ascii_hexdigit())(tg)
    // this relies on the tag not returning early
    // expecting that the &str starts with #
    // take_while max, min, expected
    // and the next 6 char is 
    // map_res to map the result to a different type
    // first &str is what is left 
    // second &str is what was parsed.
    }

fn main() {
    let a = hex_color(&"#fff23a");
    dbg!(&a);
}
