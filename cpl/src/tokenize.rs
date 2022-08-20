use std::{cell::Cell, fmt::Display};
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum Token{
    // const
    True,
    False,
    // keyword
    Use,
    Type,
    Pub,
    Fn,
    For,
    Main,
    If,
    In,
    Else,
    Let, // immut
    Var, // mut
    While,
    Break,
    Continue,
    Return,
    Const, // inline
    Static,
    Magic,
    /// 我们把print和println当作keyword处理，使语法更灵活
    Println,
    Print,
    Prelude,
    Module,
    // Reserved keywords:
    Async,
    Await,
    Become,
    Box,
    Do,
    Final,
    Goto,
    Override,
    Try,
    Virtual,
    Yield,
    // Parents
    LParent,// (
    RParent,
    LBracket,// [
    RBracket,
    LBrack, // {
    RBrack,
 // just 用于泛型
    LAngle,// <
    RAngle,// >
    // symbol
    Semicolon,// ;
    SubModule,// ::
    Colon,// :
    ThinArrow,// ->
    FatArrow,// =>
    At,// @
    Sharp,// #
    Range,// ..
    Dot,// .
    Comma,// ,
    Dollar,// $
    ExclamatoryMark,// !
    QuestionMark,// ?
    Add,// +
    Sub,// -
    Mul,// *
    Div,// /
    Mod,// %
    Pow,// ^
    RShift,// >>
    LShift,// <<
    Eq,// =
    And,// &&
    Or, // ||
    BitAnd,// &
    BitOr,// |
    // 使用cmp( left:T , right:T ) -> Order,xx.eq(),xx.ne()...进行大小判断
    // 不为它们引入新的token
    // const
    Num,
    String,
    Underline, // 特殊的Ident,会有特别的处理
    Ident,
    // Error
    Error,
}

pub struct Tokenize{
    src: Vec<u8>,
    cur: Cell<usize>,
    line: Cell<usize>
}

impl Tokenize{
    #[inline(always)]
    pub fn new(source:&str) -> Self{
        Self { src: source.into(), cur: Cell::new(0), line: Cell::new(1) }
    }
    #[inline(always)]
    fn peek(&self) -> u8{
        self.src[self.cur.get()+1]
    }
    #[inline(always)]
    fn get_is(&self,t:&[u8]) -> bool{
        let len = t.len(); 
        if self.cur.get()+len > self.src.len(){
            return false
        }
        let result = t == &self.src[self.cur.get()..self.cur.get()+len];
        if result{
            self.selfplus(len)
        }
        result
    }
    #[inline(always)]
    fn this(&self) -> u8{
        self.src[self.cur.get()]
    }
    #[inline(always)]
    fn selfplus(&self,num:usize){
        self.cur.set(self.cur.get()+num)
    }
    pub fn tokenize(&self) -> (Vec<Token>,Vec<usize>,Vec<usize>,Vec<String>){
        let mut tokens = Vec::with_capacity(self.src.len()/4);
        let mut lines = Vec::with_capacity(self.src.len()/25);
        let mut numbers:Vec<usize> = vec![];
        let mut strings : Vec<String> = vec![];

        let mut local_num = vec![];
        while self.cur.get() < self.src.len(){
            let new_char = self.src[self.cur.get()];
            tokens.push(
                match  new_char{
                    b'\n' => {
                        lines.push(tokens.len());
                        self.line.set(self.line.get()+1);
                        self.selfplus(1);
                        continue;
                    },
                    b'!' => Token::ExclamatoryMark,
                    b'@' => Token::At,
                    b'#' => Token::Sharp,
                    b'$' => Token::Dollar,
                    b'%' => Token::Mod,
                    b'^' => Token::Pow,
                    b'&' if self.peek() == b'&' => Token::And,
                    b'&' => Token::BitAnd,
                    b'|' if self.peek() == b'|' => Token::Or,
                    b'|' => Token::BitOr,
                    b'*' => Token::Mul,
                    b'(' => Token::LParent,
                    b')' => Token::RParent,
                    b'[' => Token::LBracket,
                    b']' => Token::RBracket,
                    b'{' => Token::LBrack,
                    b'}' => Token::RBrack,
                    b'-' => Token::Sub,
                    b'_' if !Self::is_ident(self.peek()) => Token::Underline,
                    b'+' => Token::Add,
                    b'=' => Token::Eq,
                    b':' => Token::Colon,
                    b';' => Token::Semicolon,
                    b',' => Token::Comma,
                    b'.' => Token::Dot,
                    b'<' => Token::LAngle,
                    b'>' => Token::RAngle,
                    b'?' => Token::QuestionMark,
                    // keywords
                    b'b' if self.get_is(b"break"   ) => Token::Break,
                    b'c' if self.get_is(b"const"   ) => Token::Const,
                    b'c' if self.get_is(b"continue") => Token::Continue,
                    b'e' if self.get_is(b"else"    ) => Token::Else,
                    b'f' if self.peek() == b'n'      => Token::Fn,
                    b'f' if self.get_is(b"for"     ) => Token::For,
                    b'f' if self.get_is(b"false"   ) => Token::False,
                    b'i' if self.peek() == b'f'      => Token::If,
                    b'i' if self.get_is(b"in"      ) => Token::In,
                    b'l' if self.get_is(b"let"     ) => Token::Let,
                    b'm' if self.get_is(b"mod"     ) => Token::Module,
                    b'm' if self.get_is(b"magic"   ) => Token::Magic,
                    b'm' if self.get_is(b"main"    ) => Token::Main,
                    b'p' if self.get_is(b"pub"     ) => Token::Pub,
                    b'p' if self.get_is(b"println" ) => Token::Println,
                    b'p' if self.get_is(b"print"   ) => Token::Print,
                    b'p' if self.get_is(b"prelude" ) => Token::Prelude,
                    b'r' if self.get_is(b"return"  ) => Token::Return,
                    b's' if self.get_is(b"static"  ) => Token::Static,
                    b't' if self.get_is(b"true"    ) => Token::True,
                    b't' if self.get_is(b"type"    ) => Token::Type,
                    b'u' if self.get_is(b"use"     ) => Token::Use,
                    b'v' if self.get_is(b"var"     ) => Token::Use,
                    b'w' if self.get_is(b"while"   ) => Token::While,
                    // reserved keywords
                    b'a' if self.get_is(b"async"   ) => Token::Async,
                    b'a' if self.get_is(b"await"   ) => Token::Await,
                    b'b' if self.get_is(b"box"     ) => Token::Box,
                    b'b' if self.get_is(b"become"  ) => Token::Become,
                    b'd' if self.peek() == b'o'      => Token::Do,
                    b'f' if self.get_is(b"final"   ) => Token::Final,
                    b'g' if self.get_is(b"goto"    ) => Token::Goto,
                    b'o' if self.get_is(b"override") => Token::Override,
                    b't' if self.get_is(b"try"     ) => Token::Try,
                    b'v' if self.get_is(b"virtual" ) => Token::Virtual,
                    b'y' if self.get_is(b"yield"   ) => Token::Yield,
                    // parse a string
                    b'"' => {
                        self.selfplus(1);
                        let mut s = vec![];
                        loop{
                            let tmp = self.this();
                            if tmp == b'"'{
                                self.selfplus(1);
                                break;
                            }
                            if tmp == b'\n'{
                                lines.push(self.line.get());
                                self.line.set(self.line.get()+1)
                            }
                            if tmp == b'\\'{
                                match self.peek() {
                                    b'r' => s.push(b'\r'),
                                    b't' => s.push(b'\t'),
                                    b'n' => s.push(b'\n'),
                                    b'\\' => s.push(b'\\'),
                                    b'"' => s.push(b'"'),
                                    other => {
                                        s.push(tmp);
                                        s.push(other)
                                    }
                                }
                                self.selfplus(2);
                                continue;
                            }
                            s.push(tmp);
                            self.selfplus(1)
                        }
                        strings.push(
                            unsafe{
                                std::mem::transmute(s)
                            }
                        );
                        tokens.push(Token::String);
                        continue;
                    }
                    b'/' if self.peek() != b'/' && self.peek() != b'*' 
                    => Token::Div,
                    b'/' if self.peek() == b'/' => {
                        self.selfplus(2);
                        while self.this() != b'\n'{
                            self.selfplus(1)
                        }
                        continue;
                    }
                    b'/' if self.peek() == b'*' => {
                        self.selfplus(2);
                        while !(self.this()== b'*' && self.peek() == b'/') {
                            if self.this() == b'\n'{
                                lines.push(self.line.get());
                                self.line.set(self.line.get()+1);
                                self.selfplus(1);
                            }
                            self.selfplus(1)
                        }
                        self.selfplus(2);
                        continue;
                    }
                    b'\r' | b'\t' | b' ' =>{
                        self.selfplus(1);
                        continue;
                    },
                    number if number >=b'0' && number <= b'9' => {
                        let num = &mut local_num;
                        num.push(number);
                        self.selfplus(1);
                        loop {
                            if self.cur.get() == self.src.len(){
                                break;
                            }
                            let tmp = self.this();
                            if tmp >= b'0' && tmp <= b'9' {
                                num.push(self.this())
                            }else if tmp == b'_'{
                            }else{
                                break;
                            }
                            self.selfplus(1)
                        }
                        numbers.push(
                            unsafe{
                                std::mem::transmute::<&[u8],&str>(&num[..]).parse().unwrap_unchecked()
                            }
                        );
                        tokens.push(Token::Num);
                        local_num.clear();
                        continue;
                    },
                    
                    ident if Self::is_ident_head(ident) => {
                        let mut new_ident = vec![];
                        new_ident.push(ident);
                        self.selfplus(1);
                        while Self::is_ident(self.this()){
                            new_ident.push(self.this());
                            self.selfplus(1)
                        }
                        strings.push(
                            unsafe{
                                std::mem::transmute(new_ident)
                            }
                        );
                        tokens.push(Token::Ident);
                        continue;
                    },
                    _ => panic!("unimpl char,in line {},curr:{},print:`{}`",self.line.get(),self.cur.get(),unsafe{std::mem::transmute::<&[u8],&str>(&[self.this()])})
                }
            );
            self.cur.set(self.cur.get()+1)
        }
        (tokens,lines,numbers,strings)
    }
    fn is_ident_head(t:u8) -> bool{
        (t >= b'a' && t <= b'z')||(t >= b'A' && t <= b'Z') || t == b'_'
    }
    fn is_ident(t:u8) -> bool{
        Self::is_ident_head(t) || (t >= b'0' && t <= b'9') || t == b'_'
    }
}

impl Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}",self))
    }
}

#[cfg(test)]
mod test{
    use super::Tokenize;

    #[test]
    fn it_works(){
        let (tokens,lines,numbers,strings) = Tokenize::new(
            include_str!("../../example/guess_number.clf")
        ).tokenize();
        println!("{:?},{:?},{:?},{:?}",tokens,lines,numbers,strings);
    }
}