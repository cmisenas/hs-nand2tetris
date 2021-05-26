mod code_module;
mod parser_module;
mod utils;

use crate::parser_module::*;
use crate::utils::*;
use std::env;

fn strip_comments(statement: String) -> String {
    statement.split("//").collect::<Vec<&str>>()[0]
        .trim()
        .to_string()
}

fn first_pass() {
    // - Create symbol table
    //   - If @var
    //   - If (LABEL)
}

fn second_pass(program: Vec<String>) -> Vec<String> {
    let mut parser = Parser::new(program.clone());
    parser.init();
    let mut result: Vec<String> = Vec::new();
    while parser.has_more_commands() {
        match parser.command_type() {
            CommandType::ACommand => result.push(parser.generate_a_ins()),
            CommandType::CCommand => result.push(parser.generate_c_ins()),
            // CommandType::LCommand => ,
            _ => panic!("Unknown command type given!"),
        }
        parser.advance();
    }
    result
}

fn get_file_name(filepath: String) -> String {
    // If you want to write to current dir instead of dir where the .asm is located
    // let index = match filepath.rfind('/') {
    //     Some(i) => i + 1,
    //     None => 0,
    // };
    // let _name = match filepath.char_indices().skip(index).next() {
    //     Some((pos, _)) => &filepath[pos..],
    //     None => "",
    // };
    filepath.replace(".asm", ".hack").to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_file = args[1].parse::<String>().expect("No program given");
    let hack_filename = get_file_name(prog_file.to_string());
    let program = read_lines(prog_file)
        .into_iter()
        // Remove empty lines or comments. Strip comments from statements too.
        .filter_map(|l| match l == "" || l.trim().starts_with("//") {
            true => None,
            _ => Some(strip_comments(l).to_string()),
        })
        .collect::<Vec<String>>();
    let result = second_pass(program);
    write_to_file(&hack_filename, &result.join("\n"));
}
