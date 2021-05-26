mod code_module;
mod utils;

use crate::code_module::*;
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

fn generate_a_ins(statement: &String) -> String {
    // Return 0xxx xxxx xxxx xxxx
    let val = statement.strip_prefix("@").unwrap().parse::<i16>().unwrap();
    to_binary(val)
}

fn to_binary(x: i16) -> String {
    // Where x is either a non-negative decimal number
    // or a symbol referring to such number.
    // NOTE: This won't work if a negative value is passed?
    format!("{:016b}", x)
}

fn is_a_ins(statement: &String) -> bool {
    statement.starts_with("@") || statement.starts_with("(")
}

fn second_pass() {
    // - Start code generation
    //   - If @, generate A instruction
    //     NOTE: (LABEL) is not expected as first_pass should've replaced it with @
    //   - Otherwise, generate C instruction
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
    let mut result: Vec<String> = Vec::new();
    for statement in program.iter() {
        if is_a_ins(statement) {
            result.push(generate_a_ins(statement));
        } else {
            result.push(generate_c_ins(&statement));
        }
    }
    write_to_file(&hack_filename, &result.join("\n"));
}
