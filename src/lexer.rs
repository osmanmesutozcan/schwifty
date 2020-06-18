use std::collections::{VecDeque, HashMap};

use crate::util::GenericError;
use std::str::from_utf8;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenType {
    TokenError,
    TokenEOF,
    TokenAtom,
    TokenConst,
    TokenNumber,
    TokenLPar,
    TokenRPar,
    TokenDot,
    TokenChar,
    TokenString,
    TokenQuote,
    TokenNewLine,
}

pub struct Builtins {}

impl Builtins {
    pub fn new() -> HashMap<String, Token> {
        let mut builtins = HashMap::new();
        builtins.insert("#t".to_string(), make_token(TokenType::TokenConst, "#t".to_string()).expect(""));
        builtins.insert("#f".to_string(), make_token(TokenType::TokenConst, "#f".to_string()).expect(""));
        builtins.insert("cons".to_string(), make_token(TokenType::TokenAtom, "cons".to_string()).expect(""));
        builtins
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Token {
    pub num: i32,
    pub typ_: TokenType,
    pub text: String,
}

pub struct Lexer {
    buf: VecDeque<u8>,
    peek: Option<u8>,
    accum_buf: Vec<u8>,
    builtins: HashMap<String, Token>,
}

impl Lexer {
    pub fn new(buf: VecDeque<u8>) -> Self {
        Lexer { buf, peek: None, accum_buf: Vec::new(), builtins: Builtins::new() }
    }

    pub fn next<'a>(&mut self) -> Result<Token, GenericError> {
        loop {
            let text = match self.next_chr() {
                Some(c) => c,
                _ => return make_token(TokenType::TokenEOF, "EOF".to_string()),
            };

            match text.as_str() {
                s if is_space(s) => continue,
                n if is_number(n) => return self.make_number(n),
                a if is_alphanumeric(a) || a == "#" => return self.make_atom_or_const(a),
                c if c == ";" => { self.skip_comment(c); continue; }
                //
                "\"" => return self.make_string(),
                "(" => return make_token(TokenType::TokenLPar, text),
                ")" => return make_token(TokenType::TokenRPar, text),
                "." => return make_token(TokenType::TokenDot, text),
                "'" => return make_token(TokenType::TokenQuote, text),
                "\n" => return make_token(TokenType::TokenNewLine, text),
                _ => return make_token(TokenType::TokenChar, text),
            };
        }
    }

    fn next_chr(&mut self) -> Option<String> {
        if self.peek.is_some() {
            let chr = self.peek;
            self.peek = None;
            return chr_to_str(chr);
        }

        chr_to_str(self.buf.pop_front())
    }

    fn peek(&mut self) -> Option<u8> {
        self.peek = self.buf.pop_front();
        self.peek
    }

    fn skip_comment(&mut self, first_char: &str) {
        self.accumulate(first_char, |c| { c != "\n" && c != "\r" });
        self.accum_buf.clear();
    }

    fn make_number(&mut self, first_char: &str) -> Result<Token, GenericError> {
        self.accumulate(first_char, |c| is_number(c));

        let accum_buf = self.accum_buf.clone();
        self.accum_buf.clear();

        let text = from_utf8(accum_buf.as_slice()).expect("Not UTF8");
        make_token(TokenType::TokenNumber, text.to_string())
    }

    // TODO: utf-16
    fn make_string(&mut self) -> Result<Token, GenericError> {
        let fst = chr_to_str(self.peek()).expect("Unexpected EOF");
        self.accumulate(fst.as_str(), |c| is_alphanumeric(c));
        self.peek();

        let accum_buf = self.accum_buf.clone();
        self.accum_buf.clear();

        let text = from_utf8(accum_buf.as_slice()).expect("Not UTF8");
        make_token(TokenType::TokenString, text.to_string())
    }

    fn make_atom_or_const(&mut self, first_char: &str) -> Result<Token, GenericError> {
        self.accumulate(first_char, |c| c.is_ascii() && !is_space(c));
        let accum_buf = self.accum_buf.clone();
        self.accum_buf.clear();

        let text = from_utf8(accum_buf.as_slice()).expect("Not UTF8");

        if self.builtins.contains_key(text) {
            return Ok(self.builtins.get(text).unwrap().clone());
        }

        make_token(TokenType::TokenAtom, text.to_string())
    }

    fn accumulate<F: FnOnce(&str) -> bool + Copy>(&mut self, first_chr: &str, predicate: F) {
        self.accum_buf.extend(Vec::from(first_chr));

        loop {
            match chr_to_str(self.peek()) {
                None => break,
                Some(next) => {
                    if !predicate(next.as_str()) {
                        break;
                    }

                    self.accum_buf.extend(next.bytes())
                }
            }
        }
    }
}

pub fn make_token<'a>(typ_: TokenType, text: String) -> Result<Token, GenericError> {
    if typ_ == TokenType::TokenNumber {
        let num = text.parse::<i32>()?;
        return Ok(Token { num, typ_: TokenType::TokenNumber, text: "".to_string() });
    }

    Ok(Token { typ_, text, num: 0 })
}

fn chr_to_str(chr: Option<u8>) -> Option<String> {
    match chr {
        None => None,
        Some(chr) => {
            return match from_utf8(&vec![chr]) {
                Err(_) => panic!("unexpected character {}", chr),
                Ok(t) => Some(t.to_string()),
            };
        }
    }
}

fn is_space(chr: &str) -> bool {
    return chr == " " || chr == "\n" || chr == "\t" || chr == "\r";
}

fn is_number(chr: &str) -> bool {
    // TODO: all numbers
    return "0" <= chr && chr <= "9";
}

fn is_alphanumeric(chr: &str) -> bool {
    let chr_ = chr.as_bytes()[0] as char;
    chr_.is_alphanumeric()
}
