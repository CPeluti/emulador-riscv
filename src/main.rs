use std::fs;
use regex::Regex;
mod parser;
mod interpreter;
mod instructions;
mod registradores;

fn main() {
    let parsed_file = parser::parse_file("src/test.txt");
    println!("{:?}", parsed_file);
    interpreter::interpret(parsed_file);
}
