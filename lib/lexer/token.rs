use nom::*;
use std::ops::{Range,RangeTo,RangeFrom,RangeFull};
use std::iter::Enumerate;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,
    // identifier and literals
    Ident(String),
    StringLiteral(String),
    IntLiteral(usize),
    BoolLiteral(bool),
    // statements
    Assign,
    If,
    Else,
    // operators
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Not,
    // reserved words
    Function,
    Let,
    Return,
    // punctuations
    Comma,
    Colon,
    SemiColon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}

#[derive(Clone,Copy,Debug)]
#[repr(C)]
pub struct Tokens<'a> {
    pub tok: &'a[Token],
    pub start: usize,
    pub end: usize,
}

impl<'a> AsChar for &'a Token {
    fn as_char(self) -> char {
        '\0'
    }
    fn is_alpha(self) -> bool {
        false
    }
    fn is_alphanum(self) -> bool {
        false
    }
    fn is_dec_digit(self) -> bool {
        false
    }
    fn is_hex_digit(self) -> bool {
        false
    }
    fn is_oct_digit(self) -> bool {
        false
    }
}

impl AsChar for Token {
    fn as_char(self) -> char {
        '\0'
    }
    fn is_alpha(self) -> bool {
        false
    }
    fn is_alphanum(self) -> bool {
        false
    }
    fn is_dec_digit(self) -> bool {
        false
    }
    fn is_hex_digit(self) -> bool {
        false
    }
    fn is_oct_digit(self) -> bool {
        false
    }
}

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tok.len()
    }
}

impl InputLength for Token {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tok: self.tok.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
      self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
  fn slice(&self, _: RangeFull) -> Self {
    Tokens {
        tok: self.tok,
        start: self.start,
        end: self.end,
    }
  }
}

impl<'a> InputIter for Tokens<'a> {
    type Item     = &'a Token;
    type RawItem  = Token;
    type Iter     = Enumerate<::std::slice::Iter<'a, Token>>;
    type IterElem = ::std::slice::Iter<'a, Token>;

    #[inline]
    fn iter_indices(&self) -> Enumerate<::std::slice::Iter<'a, Token>> {
        self.tok.iter().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> ::std::slice::Iter<'a, Token> {
        self.tok.iter()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize> where P: Fn(Self::RawItem) -> bool {
        self.tok.iter().position(|b| predicate(b.clone()))
    }
    #[inline]
    fn slice_index(&self, count:usize) -> Option<usize> {
        if self.tok.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}
