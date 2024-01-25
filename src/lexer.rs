use crate::grammar::Token;
use colored::{Colorize};

/// Struct representing a lexer for a custom language.
#[derive(Debug)]
pub struct Lexer {
    tokens_: Vec<Token>,   // Vector to store tokens
    line_num_: u32,        // Line number in the program
    line_idx_: u32,        // Character index in the current line
    brace_stack_: i32,     // Stack to track '[' and ']' pairs
    ptr_sim_: i32,         // Pointer simulation value
}

impl Lexer {
    /// Constructor to create a new Lexer instance.
    ///
    /// # Arguments
    ///
    /// * `program` - The program string to be tokenized.
    ///
    /// # Returns
    ///
    /// A new instance of `Lexer`.
    pub fn new(program: String) -> Self {
        let mut lex = Lexer {
            tokens_: vec![],
            line_num_: 1,
            line_idx_: 0,
            brace_stack_: 0,
            ptr_sim_: 0,
        };

        // Tokenize the provided program
        lex.tokenize(&program);
        lex
    }

    /// Tokenization function.
    ///
    /// # Arguments
    ///
    /// * `program` - The program string to be tokenized.
    fn tokenize(&mut self, program: &String) {
        // Iterate through characters in the program
        for (idx, c) in program.chars().enumerate() {
            self.line_idx_ += 1;
            match c {
                '>' => {
                    self.ptr_sim_ += 1;
                    self.tokens_.push(Token::MoveForward)
                }
                '<' => {
                    self.ptr_sim_ -= 1;
                    if self.ptr_sim_ < 0 {
                        self.throw_run_err(program, idx, "Index runs out of bounds");
                    }
                    self.tokens_.push(Token::MoveBack)
                }
                '+' => self.tokens_.push(Token::Add),
                '-' => self.tokens_.push(Token::Sub),
                '.' => self.tokens_.push(Token::StdOut),
                ',' => self.tokens_.push(Token::StdIn),
                '[' => {
                    self.brace_stack_ += 1;
                    self.tokens_.push(Token::LoopStart)
                }
                ']' => {
                    self.brace_stack_ -= 1;
                    if self.brace_stack_ < 0 {
                        self.throw_run_err(program, idx,
                                           "Not enough matches for ']'");
                    }
                    self.tokens_.push(Token::LoopEnd)
                }
                _ => {
                    // new_line
                    if c == Self::new_line() {
                        self.line_num_ += 1;
                        self.line_idx_ = 0;
                    }
                }
            }
        }
        // Check for unbalanced '[' brackets
        if self.brace_stack_ > 0 {
            self.throw_run_err(program, program.len() - 1,
                               &format!("An Excess of {} '[' brackets were found", self.brace_stack_));
        }
    }

    /// Function to represent a newline character.
    ///
    /// # Returns
    ///
    /// The newline character.
    fn new_line() -> char {
        0xA as char
    }

    /// Getter function to retrieve tokens.
    ///
    /// # Returns
    ///
    /// A reference to the vector of tokens.
    pub fn tokens(self) -> Vec<Token> {
        self.tokens_
    }

    /// Function to handle and print runtime errors.
    ///
    /// # Arguments
    ///
    /// * `program` - The original program string.
    /// * `lexer_idx` - The index in the lexer where the error occurred.
    /// * `message` - The error message to be displayed.
    fn throw_run_err(&self, program: &str, lexer_idx: usize, message: &str) {
        let (err_sub_str, offset) = Self::extract_err_line(program, lexer_idx);
        let space = " ".repeat(offset);
        let error = "Error".red();  // Coloring the "Error" string in red
        let line_details = format!("Line={} | Col={}", self.line_num_, self.line_idx_).bold();

        // Printing the error message with color formatting
        eprintln!(r#"
        {error}: {line_details}
            {}
            {space}^
            {space}|----- {}
        "#, err_sub_str.white(), message.red());

        // Exiting the program with an error code
        std::process::exit(1);
    }

    /// Function to extract a line containing an error.
    ///
    /// # Arguments
    ///
    /// * `program` - The original program string.
    /// * `lexer_idx` - The index in the lexer where the error occurred.
    ///
    /// # Returns
    ///
    /// A tuple containing the error substring and the offset from the start.
    fn extract_err_line(program: &str, lexer_idx: usize) -> (&str, usize) {
        let mut l_ptr = lexer_idx;
        let mut r_ptr = lexer_idx;

        // Move left to find the start of the line or newline character
        for _ in 1..10 {
            if l_ptr as i32 - 1 < 0 || program.chars().nth(l_ptr - 1).unwrap() == Self::new_line() { break; }
            l_ptr -= 1;
        }

        // Move right to find the end of the line or newline character
        for _ in 1..10 {
            if r_ptr + 1 >= program.len() || program.chars().nth(r_ptr + 1).unwrap() == Self::new_line() { break; }
            r_ptr += 1;
        }

        // Return the substring of the line and the offset from the start
        (&program[l_ptr..=r_ptr], lexer_idx - l_ptr)
    }
}
