use std::iter::Peekable;
use std::vec::IntoIter;

pub use clf_ast::*;
use crate::tokenize::Token;

pub struct Parser<'a>{
    pub tokens:Peekable<IntoIter<Token<'a>>>,
    pub slot:Vec<&'a str>
}

impl<'a> From<logos::Lexer<'a,Token<'a>>> for Parser<'a>{
    fn from(t: logos::Lexer<'a,Token<'a>>) -> Self {
        Self{
            tokens: t.collect::<Vec<Token<'a>>>().into_iter().peekable(),
            slot: Vec::new()
        }
    }
}

impl Parser<'_>{
    fn parse_expression(&mut self) -> Expression{
        match self.tokens.peek().expect("EOF") {
            Token::Num(_) => todo!(),
            _ => todo!()
        }
    }
    fn parse_for(&mut self) -> For{
        todo!()
    }
    fn parse_while(&mut self) -> While{
        todo!()
    }
    fn parse_fn(&mut self){
        todo!()
    }
    fn parse_statement(&mut self) -> Statement{
        todo!()
    }
}