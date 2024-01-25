/*
Note =  This is my interpretation for the grammar for the language
        and is not a standard in any way

    expression = Expr
    EmptyString = e
    InputSymbols = E
    Operators = Op

    E => {
        ( > ),
        ( < ),
        ( + ),
        ( - ),
        ( . ),
        ( , ),
        ( [] )
    }


    Expr => { Loop, Op }

    Expr => e

    Loop => { "[" Expr "]" }

    Op   => { ">" | "<" | "+" | "-" | "." | "," }*
 */

#[derive(Debug, Copy, Clone)]
pub enum Token {
    MoveBack = '<' as isize,
    MoveForward = '>' as isize,
    Add = '+' as isize,
    Sub = '-' as isize,
    StdOut = '.' as isize,
    StdIn = ',' as isize,
    LoopStart = '[' as isize,
    LoopEnd = ']' as isize,
}

#[derive(Debug)]
pub enum Expression {
    Loop(Vec<Expression>),
    Operator(Box<Operators>),
}

#[derive(Debug)]
pub struct Operators {
    pub _type_name: Token,
    pub _count: u32,
}