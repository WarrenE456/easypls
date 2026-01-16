use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tok {
    // Single character
    T, F, LPAREN, RPAREN,

    // Constant multicharacter
    And, Or, Not, Nor, Nand, Xor, If, Iff,

    // Variable characters
    Identifier(String),

    // Special
    EOF,
}

impl Tok {
   pub fn is_eof(&self) -> bool {
        if let Tok::EOF = self {
            true
        } else {
            false
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lexer<'a> {
    src: &'a [u8],                  // TODO support utf8 characters
    cur: usize,
    keyword_map: HashMap<String, Tok>,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(src: &'a [u8]) -> Lexer<'a> {
        let keyword_map = HashMap::from([
            (String::from("and"), Tok::And),
            (String::from("or"), Tok::Or),
            (String::from("not"), Tok::Not),
            (String::from("nor"), Tok::Nor),
            (String::from("nand"), Tok::Nand),
            (String::from("xor"), Tok::Xor),
        ]);

        Lexer { src, cur: 0, keyword_map }
    }

    fn advance(&mut self) -> Option<u8> {
        if self.cur >= self.src.len() {
            None
        } else {
            self.cur += 1;
            Some(self.src[self.cur - 1])
        }
    }

    fn peek(&mut self) -> Option<u8> {
        if self.cur >= self.src.len() {
            None
        } else {
            Some(self.src[self.cur])
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(code) = self.peek() {
            if (code as char).is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn is_match(&mut self, c: u8) -> bool {
        if self.peek().unwrap_or(0) == c {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn  lex_identifier(&mut self) -> Tok {
        let start = self.cur - 1;
        while let Some(c) = self.peek() {
            if Self::is_identifier_char(c) {
                self.advance();
            } else {
                break;
            }
        }

        let lexeme = String::from_utf8(self.src[start..self.cur].to_vec()).unwrap();

        if let Some(tok) = self.keyword_map.get(&lexeme) {
            tok.clone()
        } else {
            Tok::Identifier(lexeme)
        }
    }

    fn is_identifier_char(c: u8) -> bool {
        (c as char).is_alphanumeric() || c == b'_'
    }

    pub fn next_tok(&mut self) -> Result<Tok, String> {
        self.skip_whitespace();
        let c =  match self.advance() {
            Some(code) => code,
            None => return Ok(Tok::EOF),
        };

        match c {
            b'T' => Ok(Tok::T),
            b'F' => Ok(Tok::F),
            b'(' => Ok(Tok::LPAREN),
            b')' => Ok(Tok::RPAREN),
            b'-' => {
                if self.is_match(b'>') {
                    Ok(Tok::If)
                } else {
                    Err(format!("Expected '>' after '-' to form '->' at column {}", self.cur))
                }
            }
            b'<' => {
                if self.is_match(b'-') && self.is_match(b'>') {
                    Ok(Tok::Iff)
                } else {
                    Err(format!("Expected '->' after '<' to form '<->' at column {}", self.cur))
                }
            }
            _ => {
                if Self::is_identifier_char(c) {
                    Ok(self.lex_identifier())
                }
                else {
                    Err(format!("Unexpected character {} at column {}", c as char, self.cur))
                }
            }
        }
    }

    pub fn lex_all(&mut self) -> Result<Vec<Tok>, String> {
        let mut toks = Vec::new();
        
        loop {
            let next_tok = self.next_tok()?;
            if next_tok.is_eof() {
                break;
            }

            toks.push(next_tok);
        }

        Ok(toks)
    }
}
