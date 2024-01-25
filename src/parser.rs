use crate::grammar::{Expression, Operators, Token};
use crate::lexer::Lexer;

#[derive(Debug)]
pub struct Parser {
    tokens_: Vec<Token>,
    parser_index_: usize,
    should_optimise_: bool,
}

impl Parser {
    pub fn new(program: String, optimise: bool) -> Self {
        Parser {
            tokens_: Lexer::new(program).tokens(),
            parser_index_: 0,
            should_optimise_: optimise,
        }
    }

    pub fn generate_ast(&mut self) -> Vec<Expression> {
        let mut ast = self.parse_to_ast();
        if self.should_optimise_ {
            todo!("Fix Implementation for optimizing the ast")
            // self.optimise(&mut ast);
        }

        ast
    }

    fn parse_to_ast(&mut self) -> Vec<Expression> {
        let mut expresions: Vec<Expression> = vec![];

        while self.parser_index_ < self.tokens_.len() {
            let token = self.tokens_[self.parser_index_];
            self.parser_index_ += 1;

            expresions.push(
                match token {
                    Token::LoopStart => {
                        Expression::Loop(self.parse_to_ast())
                    }
                    Token::LoopEnd => {
                        return expresions;
                    }
                    _ => {
                        Expression::Operator(
                            Box::new(Operators {
                                type_name_: token,
                                count_: 1,
                            })
                        )
                    }
                }
            );
        }

        expresions
    }

    // fn optimise(&self, ast: &mut [Expression]) {
    //     let mut prev: Option<&mut Operators> = None;
    //     for exptr in ast {
    //         match exptr {
    //             Expression::Loop(_) => {
    //                 prev = None
    //             }
    //             Expression::Operator(new_op) => {
    //                 match prev {
    //                     Some(old_op) => {
    //                         if old_op.type_name_ == new_op.type_name_ {
    //                             old_op.count_ += 1;
    //                         }
    //                     }
    //                     None => { prev = Some(new_op) }
    //                 }
    //             }
    //         }
    //     }
    // }
}

