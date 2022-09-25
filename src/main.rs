use std::fs;
use regex::Regex;
//TODO: definir alias para regs
mod parser;
mod interpreter;
mod instructions;
mod registradores;
// vetor da memoria

fn main() {
    let parsed_file = parser::parse_file("src/test.txt");
    println!("{:?}", parsed_file);
    // println!("{:?}",parsed_file);
    // for line in 0..parsed_file.len() {
    //     println!("{}", parsed_file[line]);
    // }
    interpreter::interpret(parsed_file);
}
