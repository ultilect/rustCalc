
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    Log,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
}

#[derive(Debug,PartialOrd, PartialEq)]
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    PowLog,
    Negative,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            Add | Subtract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => PowLog,
            Log => PowLog,
            _ => DefaultZero,
        }
    }
}