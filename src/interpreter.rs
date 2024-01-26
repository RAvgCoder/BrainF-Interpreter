use std::io::Read;
use crate::grammar::{Expression, Operator, Token};

/// Struct representing an interpreter for the custom language.
#[derive(Debug)]
pub struct Interpreter<'a> {
    /// Contains the instructions to execute
    syntax_tree: &'a [Expression],
    /// The memory that the program uses
    memory_cells: Vec<u8>,
    /// Points to the index in the tape to be used
    cell_ptr: usize,
}

impl<'a> Interpreter<'a> {
    /// Maximum size of memory tape
    const MAX_SIZE: usize = u8::MAX as usize + 1;

    /// Constructs a new `Interpreter` instance.
    ///
    /// # Arguments
    ///
    /// * `ast` - The abstract syntax tree (AST) to interpret.
    ///
    /// # Returns
    ///
    /// A new instance of `Interpreter`.
    pub fn new(syntax_tree: &'a [Expression]) -> Self {
        Interpreter {
            memory_cells: vec![0; 10],
            cell_ptr: 0,
            syntax_tree,
        }
    }

    /// Runs the interpreted code.
    pub fn run_code(&mut self) {
        self.execute(self.syntax_tree, false);
    }

    /// Executes the instructions in the AST.
    ///
    /// # Arguments
    ///
    /// * `expressions` - The list of expressions to execute.
    /// * `is_in_loop` - Indicates if the expression executing is within a loop.
    fn execute(&mut self, expressions: &[Expression], is_in_loop: bool) {
        loop {
            for instruction in expressions {
                match instruction {
                    Expression::Loop(_loop) => {
                        self.execute(_loop, true);
                    }
                    Expression::Operator(_op) => {
                        self.modify_curr_cell(_op);
                    }
                }
            }

            if !is_in_loop || self.read_curr_cell() == 0 {
                break;
            }
        }
    }

    /// Modifies the memory tape based on the given operator.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator specifying the modification to perform.
    fn modify_curr_cell(&mut self, operator: &Operator) {
        match operator.type_name {
            Token::MoveBack => {
                self.cell_ptr -= operator.count;
            }
            Token::MoveForward => {
                self.cell_ptr += operator.count;
                self.grow_cell_memory();
            }
            Token::Add => {
                // Val = (CURR_NUM + COUNT) % MAX_SIZE
                let a = self.read_curr_cell() as usize + operator.count;
                let n: usize = a % Self::MAX_SIZE;
                self.write_to_cell(n as u8);
            }
            Token::Sub => {
                // Val = (CURR_NUM + MAX_SIZE - COUNT) % MAX_SIZE
                let a = Self::MAX_SIZE - operator.count;
                let b = self.read_curr_cell() as usize + a;
                let n: usize = b % Self::MAX_SIZE;
                self.write_to_cell(n as u8);
            }
            Token::StdOut => {
                print!("{}", self.read_curr_cell() as char);
            }
            Token::StdIn => {
                println!("Enter One Character");
                self.read_char();
            }
            _ => {
                eprintln!("Token: {:?}, cannot modify the memory", operator);
            }
        }
    }

    /// Reads a character from standard input and writes it to memory.
    fn read_char(&mut self) {
        let mut buffer = [0; 1];
        match std::io::stdin().read(&mut buffer) {
            Ok(_) => {
                if let Some(character) = buffer[0].into() {
                    println!("You entered: {}={}", character as char, character);
                    self.write_to_cell(character);
                } else {
                    println!("Invalid character entered");
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
            }
        }
    }

    /// Reads a value from the memory tape at the current pointer position.
    ///
    /// # Returns
    ///
    /// The value read from memory.
    fn read_curr_cell(&self) -> u8 {
        self.memory_cells[self.cell_ptr]
    }

    /// Writes a value to the memory tape at the current pointer position.
    ///
    /// # Arguments
    ///
    /// * `num` - The value to write to memory.
    fn write_to_cell(&mut self, num: u8) {
        self.memory_cells[self.cell_ptr] = num;
    }

    /// Grows the memory tape if the current pointer exceeds its size.
    fn grow_cell_memory(&mut self) {
        if self.cell_ptr == self.memory_cells.len() {
            for _ in 0..10 {
                self.memory_cells.push(0);
            }
        }
    }
}
