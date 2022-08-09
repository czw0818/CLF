pub struct AST(Vec<Statement>);

/// This is Statement which will not return any value
#[derive(Debug)]
pub enum Statement{
    Expr(Expression),
    Let(u32,Expression),
}

/// This is Expression,All Expression will return a value
#[derive(Debug)]
pub enum Expression{
    // operations
    BinOp(Box<BinOp>),
    LeftValueOp(Box<LeftValueOp>),
    // Call Function
    Call(usize),
    // read value
    Const(Value),
    Get(u32),
    // returns never
    Break,
    Continue,
    Return(Option<Box<Expression>>),
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
    Plus(Expression,Expression),
    // +
    Sub(Expression,Expression),
    // -
    Mul(Expression,Expression),
    // *
    Div(Expression,Expression),
    // /
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
    // && &
    Or(Expression,Expression), // || |
}

/// TODO!
pub enum Declare{}

/// Block is used to store statements and clean up some variables
#[derive(Debug)]
pub struct Block{
    pub slot:Option<Vec<u32>>,
    pub states:Vec<Statement>,
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
    pub slot:u32,
    pub iter:Expression,
    pub block:Block,
}
 
#[derive(Debug)]
pub enum Value{
    Bool(bool),
    Int(usize),
    // unimpl:float
}