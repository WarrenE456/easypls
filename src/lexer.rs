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
    EOL,
}

impl Default for Tok {
    fn default() -> Self {
        Tok::EOL
    }
}

impl Tok {
   pub fn is_eol(&self) -> bool {
        if let Tok::EOL = self {
            true
        } else {
            false
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Tok::T => String::from("T"),
            Tok::F => String::from("F"),
            Tok::LPAREN => String::from("("),
            Tok::RPAREN => String::from(")"),
            Tok::And => String::from("and"),
            Tok::Or => String::from("or"),
            Tok::Not => String::from("not"),
            Tok::Nor => String::from("nor"),
            Tok::Nand => String::from("nand"),
            Tok::Xor => String::from("xor"),
            Tok::If => String::from("->"),
            Tok::Iff => String::from("<->"),
            Tok::Identifier(name) => name.clone(),
            Tok::EOL => String::from("end of file"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Lexer<'a> {
    src: &'a [u8],                  // TODO support utf8 characters
    cur: usize,
    keyword_map: HashMap<String, Tok>,
    tok_next: Result<Tok, String>,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(src: &'a [u8]) -> Result<Lexer<'a>, String> {
        let keyword_map = HashMap::from([
            (String::from("and"), Tok::And),
            (String::from("or"), Tok::Or),
            (String::from("not"), Tok::Not),
            (String::from("nor"), Tok::Nor),
            (String::from("nand"), Tok::Nand),
            (String::from("xor"), Tok::Xor),
        ]);

        let mut lexer = Lexer { src, cur: 0, keyword_map, tok_next: Ok(Tok::EOL) };
        lexer.tok_next = lexer.next_tok();

        Ok(lexer)
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

    fn is_match(&mut self, c: u8) -> bool {
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

    fn next_tok(&mut self) -> Result<Tok, String> {
        self.skip_whitespace();
        let c =  match self.advance() {
            Some(code) => code,
            None => return Ok(Tok::EOL),
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

    pub fn advance_tok(&mut self) -> Result<Tok, String> {
        let next = self.tok_next.clone();
        self.tok_next = self.next_tok();

        next
    }

    pub fn peek_tok(&self) -> Result<Tok, String> {
        self.tok_next.clone()
    }

    pub fn lex_all(&mut self) -> Result<Vec<Tok>, String> {
        let mut toks = Vec::new();
        
        loop {
            let next_tok = self.advance_tok()?;
            if next_tok.is_eol() {
                break;
            }

            toks.push(next_tok);
        }

        Ok(toks)
    }
}
