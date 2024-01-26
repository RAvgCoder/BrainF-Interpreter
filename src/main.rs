use std::fs::File;
use std::io::Read;
use std::time::Instant;
use colored::Colorize;
use crate::interpreter::Interpreter;
use crate::parser::Parser;

mod lexer;
mod grammar;
mod parser;
mod interpreter;

struct Program {
    content: String,
    should_optimise: bool,
}

fn main() {
    // Read the program from file
    let Program { content, should_optimise } = read_file();

    // Init a parser that takes the program and converts it to a token stream
    let mut parser = Parser::new(content.clone(), should_optimise);
    let mut parser_unop = Parser::new(content.clone(), false);


    time_exec(&mut parser, "Optimised");
    println!("\n");
    time_exec(&mut parser_unop, "Un_Optimised");
}

fn time_exec(parser: &mut Parser, name: &str) {
    println!("|---------[{}]---------|", name);
    print!("\t");
    let start = Instant::now();

    // Generates an abstract syntax tree for the program
    parser.generate_syntax_tree();

    // Creates the interpreter and runs the code
    create_interpreter(parser).run_code();

    // Times the run
    let duration = start.elapsed();
    println!("	Time taken is: {:?} | Instruction count {:?}", duration, parser.get_num_of_instr());
    println!("|-------------------------|\n");
}

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
        Err(why) => panic!("couldn't open {}: {}", file_path, why)
    };

    let mut prog = String::new();
    match file.read_to_string(&mut prog) {
        Ok(_) => {}
        Err(why) => panic!("couldn't read {}: {}", file_path, why)
    }

    Program { content: prog, should_optimise: optimisation }
}
