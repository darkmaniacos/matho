mod ast;
mod parser;
mod token;

use crate::ast::AST;
use crate::parser::Parser;
use crate::token::Tokenizer;

fn main() {
    let mut input = String::new();
    let err = std::io::stdin().read_line(&mut input);

    let mut late_print = input.clone();
    late_print.truncate(input.len()-2);
    
    match err {
        Err(e) => {
            println!("ERROR: {}", e.to_string());
            return;
        },
        _ => {}
    }

    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    
    let mut parser = Parser::new(tokens.iter());
    let syntax_tree = parser.parse();

    /* Descomente esse codigo para mostrar informações sobre o processamento dos dados: */
    // println!("{:#?}", syntax_tree);
    // println!("input -> '{}'", late_print.as_str());
    println!("evalution result -> '{}'", AST::evaluate(&syntax_tree));
}
