mod code_module;
mod parser_module;
mod symbol_table_module;
mod utils;

use crate::parser_module::*;
use crate::symbol_table_module::*;
use crate::utils::*;
use std::env;

fn strip_comments(statement: String) -> String {
    statement.split("//").collect::<Vec<&str>>()[0]
        .trim()
        .to_string()
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
    let raw_program = read_lines(prog_file)
        .into_iter()
        // Remove empty lines or comments. Strip comments from statements too.
        .filter_map(|l| match l == "" || l.trim().starts_with("//") {
            true => None,
            _ => Some(strip_comments(l).to_string()),
        })
        .collect::<Vec<String>>();

    // First pass
    // Will be the counter of each program line (without labels)
    let mut program: Vec<String> = Vec::new();
    let mut first_parser = Parser::new(raw_program.clone());
    let mut program_index = 0;
    let mut symbol_table = SymbolTable::new();
    while first_parser.has_more_commands() {
        match first_parser.command_type() {
            CommandType::ACommand | CommandType::CCommand => {
                program.push(first_parser.current_command().to_string());
                program_index += 1;
            }
            CommandType::LCommand => {
                let mut l_command = "@".to_string();
                l_command.push_str(
                    first_parser
                        .current_command()
                        .trim_matches(|c| c == ')' || c == '('),
                );
                // Add Label entry to symbol with program_index
                symbol_table.add_entry(&l_command, program_index.to_string());
            }
        }
        first_parser.advance();
    }

    // Second pass
    let mut second_parser = Parser::new(program.clone());
    second_parser.init();
    let mut result: Vec<String> = Vec::new();
    while second_parser.has_more_commands() {
        let current_command = second_parser.current_command();
        match second_parser.command_type() {
            CommandType::ACommand => {
                if !current_command.chars().skip(1).all(char::is_numeric)
                    && !symbol_table.contains(current_command)
                {
                    // Variable found!
                    println!("Variable found! {}", current_command);
                    // Add entry to symbol with program_index if not yet added
                    symbol_table.add_var_entry(current_command);
                }
                if symbol_table.contains(current_command) {
                    let mut symbol_val = "@".to_string();
                    symbol_val.push_str(&symbol_table.get_address(current_command));
                    second_parser.replace_current_command(symbol_val.to_string());
                }
                result.push(second_parser.generate_a_ins());
            }
            CommandType::CCommand => result.push(second_parser.generate_c_ins()),
            _ => panic!("Unknown command type given!"),
        }
        second_parser.advance();
    }
    write_to_file(&hack_filename, &result.join("\n"));
}
