use crate::operator::Operator;
use crate::token::TokenType;

/// プログラム
pub struct Program {
    pub statements: Vec<Statement>,
}

/// 文
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
}

// let文
pub struct LetStatement {
    pub token_type: TokenType,
    pub identifier: Identifier,
    pub expression: Expression,
}

// Return文
pub struct ReturnStatement {
    pub token_type: TokenType,
    pub expression: Expression,
}

/// 式
#[derive(Debug, PartialEq)]
pub enum Expression {
    /// 整数リテラル
    IntegerLiteral(u32),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
}

/// 前置演算式
#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub operator: Operator,
    pub right: Box<Expression>,
}

/// 中置演算式
#[derive(Debug, PartialEq)]
pub struct InfixExpression {
    pub operator: Operator,
    pub right: Box<Expression>,
    pub left: Box<Expression>,
}

/// 式文
pub struct ExpressionStatement {
    pub expression: Expression,
}

/// 識別子
pub struct Identifier {
    pub token_type: TokenType,
    pub value: String,
}
