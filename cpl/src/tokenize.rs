use logos::Logos;

/// 使用Logos进行tokenize
#[derive(Debug,Logos,PartialEq)]
pub enum Token<'a>{
    #[token("true")]
    True,

    #[token("false")]
    False,
    
    #[token("use")]
    Use,

    #[token("type")]
    Type,

    #[token("pub")]
    Pub,

    #[token("fn")]
    Fn,

    #[token("main")]
    Main,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("let")]
    Let, // immut

    #[token("var")]
    Var, // mut

    #[token("while")]
    While,

    #[token("break")]
    Break,

    #[token("cotinue")]
    Continue,

    #[token("return")]
    Return,

    #[token("const")]
    Const, // inline

    #[token("static")]
    Static,
    /// 我们把print和println当作keyword处理，使语法更灵活
    #[token("println")]
    Println,

    #[token("print")]
    Print,

    #[token("prelude")]
    Prelude,

    #[token("mod")]
    Module,

    #[token("(")]
    LParent,

    #[token(")")]
    RParent,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("{")]
    LBrack,

    #[token("}")]
    RBrack,

    #[token("<")] // just 用于泛型
    LAngle,

    #[token(">")]
    RAngle,

    #[token(";")]
    Semicolon,

    #[token("::")]
    SubModule,

    #[token(":")]
    Colon,

    #[token("->")]
    ThinArrow,

    #[token("=>")]
    FatArrow,

    #[token("@")]
    At,

    #[token("#")]
    Sharp,

    #[token("..")]
    Range,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("$")]
    Dollar,

    #[token("!")]
    ExclamatoryMark,

    #[token("?")]
    QuestionMark,

    #[token("+")]
    Add,

    #[token("-")]
    Sub,

    #[token("*")]
    Mul,

    #[token("/")]
    Div,

    #[token("%")]
    Mod,

    #[token("^")]
    Pow,

    #[token(">>")]
    RShift,

    #[token("<<")]
    LShift,

    #[token("=")]
    Eq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    #[token("&")]
    BitAnd,

    #[token("|")]
    BitOr,
    // 使用cmp( left:T , right:T ) -> Order,xx.eq(),xx.ne()...进行大小判断
    // 不为它们引入新的token

    #[regex("[0-9]" , |lexer|lexer.slice().parse())]
    Num(usize),

    #[regex(r#""((\\")|[^"])*""# , |lexer|Some(lexer.slice()))]
    String(&'a str),

    #[token("_")]
    Underline, // 特殊的Ident,会有特别的处理

    #[regex(r"[\p{Han}\p{Latin}][_\p{Han}\p{Latin}\p{Number}]*" , |lexer|Some(lexer.slice()))]
    Ident(&'a str),

    #[error]
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Error,
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn tokenize_net_server(){
        let ts = Token::lexer(include_str!("../../example/net_server.clf"));
        for t in ts{
            println!("{:?}",t);
            assert_ne!(t,Token::Error)
        }
    }
    #[test]
    fn tokenize_guess_number(){
        let mut ts = Token::lexer(include_str!("../../example/guess_number.clf"));
        while let Some(t) = ts.next(){
            println!("{:?},{:?},{}",t,ts.span(),ts.slice());
            assert_ne!(t,Token::Error);
        }
    }
}