/// This is Expression,All Expression will return a value

type Type = String;
#[derive(Debug)]
pub enum Expression{
    // operations
    BinOp(Box<BinOp>),
    LeftValueOp(Box<LeftValueOp>),
    // Call Function
    Call(String,Vec<Expression>),
    CallExpression(Box<Expression>,Vec<Expression>),
    // read value
    Const(Value),
    Get(u32),
    // returns never
    Break,
    Continue,
    Return(Option<Box<Expression>>),
    IfElse(Box<IfElse>),
    While(Box<While>),
    For(Box<For>),
    Let(Box<(u32,Expression,Option<Type>)>),
    Var(Box<(u32,Expression,Option<Type>)>)
}

#[derive(Debug)]
pub enum LeftValueOp {
    Assign(Expression),
    PlusEq(Expression),
    SubEq(Expression),
    PlusOne,
    SubOne,
}

#[derive(Debug)]
pub enum BinOp {
    // calc : num to num
    Add(Expression,Expression),
    // +
    Sub(Expression,Expression),
    // -
    Mul(Expression,Expression),
    // *
    Div(Expression,Expression),
    // /
    LShift(Expression,Expression),
    // <<
    RShift(Expression,Expression),
    // >>
    BitAnd(Expression,Expression),
    // &
    BitOr(Expression,Expression),
    // |
    Pow(Expression,Expression),
    // ^
    // compare : num to bool
    Gt(Expression,Expression),
    // >
    Lt(Expression,Expression),
    // <
    GtEq(Expression,Expression),
    // >=
    LtEq(Expression,Expression),
    // <=
    // eqs : basic/ref to bool
    Eqs(Expression,Expression),
    // ==
    NotEq(Expression,Expression),
    // !=
    // logic : bool to bool
    And(Expression,Expression),
    // &&
    Or(Expression,Expression), 
    // ||
}

/// TODO!
pub struct NewType{
    pub name: String,
    pub fields:Vec<(String,Type)>,
    pub methods:Vec<usize>,
    pub which_type:NewTypeType
}
pub enum NewTypeType{
    Struct,
    Union,
    Enum,
}
/// Block is used to store statements and clean up some variables
#[derive(Debug)]
pub struct Block{
    pub states:Vec<Expression>,
}

impl From<IfElse> for Expression{
    fn from(from: IfElse) -> Self {
        Expression::IfElse(Box::new(from))
    }
}
impl From<While> for Expression{
    fn from(from: While) -> Self{
        Expression::While(Box::new(from))
    }
}
impl From<For> for Expression{
    fn from(from: For) -> Self {
        Expression::For(Box::new(from))
    }
}
impl Into<Block> for Expression{
    fn into(self) -> Block {
        Block { states: vec![self] }
    }
}
#[derive(Debug)]
pub struct IfElse{
    pub condition:Expression, // type:bool
    pub true_block:Block,
    pub false_block:Option<Block>, // maybe without else
}

#[derive(Debug)]
pub struct While{
    pub condition:Expression, // typr:bool
    pub cric:Block,  // the Circulatory body
}

#[derive(Debug)]
pub struct For{
    pub slot:String,
    pub iter:Expression,
    pub block:Block,
}
 
#[derive(Debug)]
pub enum Value{
    Bool(bool),
    Int(usize),
    Var(String),
    String(String),
    Float(usize,usize),
}

