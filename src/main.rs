use std::{env::args, fs};
mod lexer;
mod tokenize;
use lexer::Lexer;
use tokenize::tokenizer;

fn main(){
    
    let file_name = args().nth(1);


    let file = if let Some(f) = file_name {
        f
    } else {
        panic!("Excpected File ...");
    };


    let content = fs::read_to_string(file);



    let cont = if content.is_ok() {
        content.unwrap()
    } else {
        panic!("Could not open file for reading ...");
    };



    // let res = tokenizer(cont).unwrap();
    // println!("{:#?}", res);
    let mut lexer = Lexer::new(cont);
    

    lexer.lex();
 
}
