use joker::token::{Token, TokenData};
use result::Result;
use error::Error;
use parser::Parser;

pub trait State {
    fn skip(&mut self) -> Result<()>;
    fn read(&mut self) -> Result<Token>;
    fn read_op(&mut self) -> Result<Token>;
    fn peek(&mut self) -> Result<&Token>;
    fn peek_op(&mut self) -> Result<&Token>;
    fn expect(&mut self, expected: TokenData) -> Result<Token>;
    fn matches_token(&mut self, expected: TokenData) -> Result<Option<Token>>;
    fn matches(&mut self, expected: TokenData) -> Result<bool>;
    fn matches_op(&mut self, expected: TokenData) -> Result<bool>;
    fn reread(&mut self, expected: TokenData) -> Token;
    fn has_arg_same_line(&mut self) -> Result<bool>;
}

impl<I: Iterator<Item=char>> State for Parser<I> {
    fn skip(&mut self) -> Result<()> {
        self.lexer.skip_token().map_err(Error::LexError)
    }

    fn read(&mut self) -> Result<Token> {
        self.lexer.read_token().map_err(Error::LexError)
    }

    fn read_op(&mut self) -> Result<Token> {
        let mut cx = self.shared_cx.get();
        cx.operator = true;
        self.shared_cx.set(cx);
        let result = self.lexer.read_token().map_err(Error::LexError);
        let mut cx = self.shared_cx.get();
        cx.operator = false;
        self.shared_cx.set(cx);
        result
    }

    fn peek(&mut self) -> Result<&Token> {
        self.lexer.peek_token().map_err(Error::LexError)
    }

    fn peek_op(&mut self) -> Result<&Token> {
        let mut cx = self.shared_cx.get();
        cx.operator = true;
        self.shared_cx.set(cx);
        let result = self.lexer.peek_token().map_err(Error::LexError);
        let mut cx = self.shared_cx.get();
        cx.operator = false;
        self.shared_cx.set(cx);
        result
    }

    fn expect(&mut self, expected: TokenData) -> Result<Token> {
        let token = try!(self.read());
        if token.value != expected {
            return Err(Error::UnexpectedToken(token));
        }
        Ok(token)
    }

    fn matches_token(&mut self, expected: TokenData) -> Result<Option<Token>> {
        let token = try!(self.read());
        if token.value != expected {
            self.lexer.unread_token(token);
            return Ok(None);
        }
        Ok(Some(token))
    }

    fn matches(&mut self, expected: TokenData) -> Result<bool> {
        let token = try!(self.read());
        if token.value != expected {
            self.lexer.unread_token(token);
            return Ok(false);
        }
        Ok(true)
    }

    fn matches_op(&mut self, expected: TokenData) -> Result<bool> {
        let token = try!(self.read_op());
        if token.value != expected {
            self.lexer.unread_token(token);
            return Ok(false);
        }
        Ok(true)
    }

    fn reread(&mut self, expected: TokenData) -> Token {
        debug_assert!(self.lexer.repeek_token().value == expected);
        self.lexer.reread_token()
        // debug_assert!(self.peek().map(|actual| actual.value == expected).unwrap_or(false));
        // self.read().unwrap()
    }

    fn has_arg_same_line(&mut self) -> Result<bool> {
        let next = try!(self.peek());
        Ok(!next.newline && next.value != TokenData::Semi && next.value != TokenData::RBrace)
    }
}
