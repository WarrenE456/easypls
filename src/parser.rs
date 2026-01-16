use crate::lexer::{Lexer, Tok};
use crate::expr::Expr;

// Grammar
//
//  statement -> expr$
//  expr -> iff
//  iff ->  if ("<->" if)*
//  if ->   or ("->" or)*
//  or ->   xor ("or" xor)*
//  xor ->  and ("xor" and)
//  and ->  not_compound ("and" not_compound)*
//  not_compound -> not (("nand" | "nor") not_compound)?
//  not ->  not? atom
//  group -> "(" expr ")" | atom
//  atom -> "T" | "F" | IDENTIFIER
//

#[allow(dead_code)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn from(lexer: Lexer<'a>) -> Parser<'a> {
        Parser { lexer }
    }

    fn is_match(&mut self, t: Tok) -> Result<bool, String> {
        if self.lexer.peek_tok()? == t {
            self.lexer.advance_tok()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn statement(&mut self) -> Result<Expr, String> {
        let expr = self.expr()?;

        // Check if final token is not an EOL token
        if !self.lexer.advance_tok()?.is_eol() {
            Err(format!("Expected end of line"))
        } else {
            Ok(expr)
        }
    }

    fn expr(&mut self) -> Result<Expr, String> {
        self.iff()
    }

    fn iff(&mut self) -> Result<Expr, String> {
        let mut expr = self.eif()?;
        while self.is_match(Tok::Iff)? {
            expr = Expr::iff(expr, self.eif()?);
        }
        Ok(expr)
    }

    // if misspelled
    fn eif(&mut self) -> Result<Expr, String> {
        let mut expr = self.or()?;
        while self.is_match(Tok::If)? {
            expr = Expr::eif(expr, self.or()?);
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.xor()?;
        while self.is_match(Tok::Or)? {
            expr = Expr::or(expr, self.xor()?);
        }
        Ok(expr)
    }

    fn xor(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        while self.is_match(Tok::Xor)? {
            expr = Expr::xor(expr, self.and()?);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.not_compound()?;
        while self.is_match(Tok::And)? {
            expr = Expr::and(expr, self.not_compound()?);
        }
        Ok(expr)
    }

    fn not_compound(&mut self) -> Result<Expr, String> {
        let l = self.not()?;

        let peek = self.lexer.peek_tok()?;
        if peek == Tok::Nand {
            Ok(Expr::nand(l, self.not_compound()?))
        }
        else if peek == Tok::Nor {
            Ok(Expr::nor(l, self.not_compound()?))
        } else {
            Ok(l)
        }
    }

    fn not(&mut self) -> Result<Expr, String> {
        if self.is_match(Tok::Not)? {
            Ok(Expr::not(self.atom()?))
        } else {
            self.atom()
        }
    }

    fn group(&mut self) -> Result<Expr, String> {
        if self.is_match(Tok::LPAREN)? {
            let expr = self.expr()?;

            if !self.is_match(Tok::RPAREN)? {
                Err(format!("Expected closing parenthesis, found '{}'", self.lexer.peek_tok()?.to_string()))
            } else {
                Ok(expr)
            }
        } else {
            self.atom()
        }
    }

    fn atom(&mut self) -> Result<Expr, String> {
        match self.lexer.advance_tok()? {
            Tok::T => todo!(),
            Tok::F => todo!(),
            Tok::Identifier(name) => Ok(Expr::Var(name)),
            _ => Err(format!("Expected literal or identifier, found '{}'", self.lexer.peek_tok()?.to_string())),
        }
    }
}
