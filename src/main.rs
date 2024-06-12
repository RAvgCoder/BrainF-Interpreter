use std::fs::File;
use std::io::Read;

use colored::Colorize;

use crate::interpreter::Interpreter;
use crate::parser::Parser;

mod grammar;
mod interpreter;
mod lexer;
mod parser;

/// Represents a Brainfuck program with its content and optimization flag.
struct Program {
    content: String,
    should_optimise: bool,
}

/// Main entry point of the program. Reads a Brainfuck program from a file, parses it,
/// optimizes it, and executes it using an interpreter.
fn main() {
    // Read the program from file
    let Program {
        content,
        should_optimise,
    } = read_file();

    // Init a parser that takes the program and converts it to a token stream
    let mut parser_optimised = Parser::new(content.clone(), should_optimise);
    let mut parser_unoptimised = Parser::new(content.clone(), should_optimise);

    // Generates an abstract syntax tree for the program
    parser_optimised.generate_syntax_tree();
    parser_unoptimised.generate_syntax_tree();

    let op_inst = parser_optimised.get_num_of_instr();
    let unop_inst = parser_unoptimised.get_num_of_instr();
    let percentage_diff = (unop_inst as f32 / op_inst as f32) * 100.0;
    println!(
        r#"
| Number of instructions to execute:
| Optimised: {}
| Unoptimised: {}
| Persentage diff in token size : {}%
    "#,
        op_inst, unop_inst, percentage_diff
    );

    // Creates the interpreter to run the code
    let mut interpreter = create_interpreter(&mut parser_optimised);

    // Executes the code
    interpreter.run_code();
}

/// Creates an interpreter for the given parser and returns it.
fn create_interpreter(parser: &mut Parser) -> Interpreter {
    match parser.get_ast() {
        Some(syntax_tree) => {
            return Interpreter::new(syntax_tree);
        }
        None => {
            eprintln!("{}", "Tree has not been generated yet".red());
            std::process::exit(255);
        }
    }
}

/// Reads a Brainfuck program from a file and returns a `Program` struct containing
/// the program content and optimization flag.
fn read_file() -> Program {
    // let args: Vec<String> = env::args().collect();
    //
    // if args.len() < 2 {
    //     eprintln!("Usage: {} <filename> <flag[-0]>", args[0]);
    //     std::process::exit(404);
    // }
    //
    // let file_path = &args[1];
    // let optimisation = args.len() > 2;

    let file_path = String::from("resources/program.bfk");
    let optimisation = true;

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open {}: {}", file_path, why),
    };

    let mut prog = String::new();
    match file.read_to_string(&mut prog) {
        Ok(_) => {}
        Err(why) => panic!("couldn't read {}: {}", file_path, why),
    }

    Program {
        content: prog,
        should_optimise: optimisation,
    }
}
