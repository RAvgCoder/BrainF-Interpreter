use crate::grammar::{Expression, Operator, Token};
use crate::lexer::Lexer;

/// Struct representing a parser for the custom language.
#[derive(Debug)]
pub struct Parser {
    lexer_: Lexer,              // Lexer instance to tokenize the program
    parser_index_: usize,       // Index to keep track of parsing progress
    should_optimise_: bool,     // Flag indicating whether to optimize the AST
}

impl Parser {
    /// Constructs a new `Parser` instance.
    ///
    /// # Arguments
    ///
    /// * `program` - The program string to be parsed.
    /// * `optimise` - A boolean indicating whether to optimize the AST.
    ///
    /// # Returns
    ///
    /// A new instance of `Parser`.
    pub fn new(program: String, optimise: bool) -> Self {
        Parser {
            lexer_: Lexer::new(program),
            parser_index_: 0,
            should_optimise_: optimise,
        }
    }

    /// Generates the abstract syntax tree (AST) by parsing the program.
    ///
    /// # Returns
    ///
    /// The AST represented as a vector of `Expression`.
    pub fn generate_ast(&mut self) -> Vec<Expression> {
        let mut ast = self.parse_to_ast();
        if self.should_optimise_ {
            todo!("Fix Implementation for optimizing the ast")
            // self.optimise(&mut ast);
        }

        ast
    }

    /// Parses the tokens into an abstract syntax tree (AST).
    ///
    /// # Returns
    ///
    /// The AST represented as a vector of `Expression`.
    fn parse_to_ast(&mut self) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = vec![];

        while self.parser_index_ < self.lexer_.tokens().len() {
            let token = self.lexer_.tokens()[self.parser_index_];
            self.parser_index_ += 1;

            expressions.push(
                match token {
                    Token::LoopStart => {
                        Expression::Loop(self.parse_to_ast())
                    }
                    Token::LoopEnd => {
                        return expressions;
                    }
                    _ => {
                        Expression::Operator(
                            Box::new(Operator {
                                type_name_: token,
                                count_: 1,
                            })
                        )
                    }
                }
            );
        }

        expressions
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
