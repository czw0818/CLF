use std::cell::Cell;

pub use clf_ast::ast::*;
use crate::tokenize::Token;

pub struct Parser{
    pub tokens: Vec<Token>,
    pub lines : Vec<usize>,
    pub strings:Vec<String>,
    pub numbers:Vec<usize>,
    pub cur_str   : Cell<usize>,
    pub cur_number: Cell<usize>,
    pub curr      : Cell<usize>,
}

impl Parser {
    pub fn new(t:Vec<Token>,lines:Vec<usize>,strings:Vec<String>,numbers:Vec<usize>) -> Self{
        Self { 
            tokens: t,
            lines: lines,
            strings: strings, 
            numbers: numbers, 
            cur_str: Cell::default(), 
            cur_number: Cell::default(), 
            curr: Cell::default() }
    }
    #[inline(always)]
    fn peek_is(&self,t:Token) -> bool{
        self.peek() == &t
    }
    #[inline(always)]
    fn assert_next(&self,t:Token){
        if !self.next_is(t){
            let mut index = 0 ;
            while self.lines[index] <= self.curr.get(){
                index += 1
            }
            eprintln!("line {index},the {} Token,expect:{},found{}",self.curr.get() - self.lines[index-1],t,self.this());
            panic!()
        }
    }
    #[inline(always)]
    fn next_is(&self,t:Token) -> bool{
        self.next() == &t
    }
    #[inline(always)]
    fn next(&self) -> &Token{
        self.curr.set(self.curr.get()+ 1);
        self.tokens.get(self.curr.get()).unwrap()
    }
    #[inline(always)]
    fn peek(&self) -> &Token{
        self.tokens.get(self.curr.get()+1).unwrap()
    }
    #[inline]
    fn this(&self) -> &Token{
        self.tokens.get(self.curr.get()).unwrap_or_else(
            ||{
                eprintln!(
                    "index out of tokens,now is {},but length is {}",
                    self.curr.get(),
                    self.tokens.len()
                );
                panic!()
            }
        )
    }
    #[inline(always)]
    fn goto(&self,index:usize){
        self.curr.set(index)
    }
    pub fn parse_expression(&mut self) -> Expression{
        todo!()
    }
    fn parse_ifelse(&mut self) -> IfElse{
        self.assert_next(Token::If);
        let condition = self.parse_expression();
        let block = self.parse_block();
        let else_block: Block;
        if self.peek_is(Token::Else){
            self.next();
            if self.peek_is(Token::If){
                else_block = Expression::into(
                    Expression(self.parse_ifelse().into(),Type::Undefined)
                );
            }else{
                else_block = self.parse_block()
            }
            return IfElse{
                condition:condition,
                true_block:block,
                false_block:Some(else_block)
            }
        }else{
            return IfElse{
                condition:condition,
                true_block:block,
                false_block:None
            }
        }
    }
    fn parse_block(&self) -> Block{
        todo!()
    }
    fn parse_for(&self) -> For{
        todo!()
    }
    fn parse_while(&self) -> While{
        todo!()
    }
    fn parse_fn(&self){
        todo!()
    }
    fn parse_expressions(&self) -> Vec<Expression>{
        todo!()
    }
}

#[cfg(test)]
mod test{
    use crate::{Parser, tokenize::{Tokenize, Token}};

    #[test]
    #[should_panic]
    fn test_error_system(){
        let (tokens,lines,numbers,strings) = Tokenize::new(
            include_str!("../../example/guess_number.clf")
        ).tokenize();
        let parser = Parser::new(
            tokens,lines,strings,numbers
        );
        parser.assert_next(Token::String);
    }
}