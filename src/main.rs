use std::env;
use std::fs::File;
use std::io::Read;
use crate::parser::Parser;

mod lexer;
mod grammar;
mod parser;

struct Program {
    content: String,
    should_optimise: bool,
}

fn main() {
    let Program { content, should_optimise } = read_file();
    let mut parser = Parser::new(content, should_optimise);

    let ast = parser.generate_ast();


    for e in &ast {
        println!("{:#?}", e);
    }


    println!("{:#?}", parser);
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
    let optimisation = false;

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
