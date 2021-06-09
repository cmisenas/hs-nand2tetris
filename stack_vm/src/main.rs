mod code_writer_module;
mod parser_module;
mod stack;
mod utils;

use crate::code_writer_module::*;
use crate::parser_module::*;
use crate::stack::*;
use crate::utils::*;
use std::env;
use std::fs;

fn get_file_name(filepath: String) -> String {
    filepath.replace(".vm", ".asm").to_string()
}

/**
 * RAM address
 * 0-15             16 virtual registers
 *      RAM[0]      SP      Stack pointer: points to the next topmost location in the stack
 *      RAM[1]      LCL     Points to the base of the current VM function's local segment
 *      RAM[2]      ARG     Points to the base of the current VM function's argument segment
 *      RAM[3]      THIS    Points to the base of the current this segment (within the heap)
 *      RAM[4]      THAT    Points to the base of the current that segment (within the heap)
 *      RAM[5-12]           Holds the contents of the temp segment
 *      RAM[13-15]          Can be used by the VM implementation as general-purpose registers
 * 16-255           Static variables (of all the VM functions in the VM program)
 * 256-2047         Stack
 * 2048-16483       Heap (used to store objects and arrays)
 * 16384-24575      Memory mapped I/O
 */

/**
 * From the book: The main program should construct:
 * - a Parser to parse the VM input file
 * - a CodeWriter to generate code into the corresponding output file
 * It should then march through the VM commands in the input file and
 * generate assembly code for each one of them.
 *
 * If the program's argument is a directory name rather than a file name,
 * the main program should process all the .vm files in this directory.
 * In doing so, it should use a separate Parser for handling each input file
 * and a single CodeWriter for handling the output.
 */
fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_arg = args[1].parse::<String>().expect("No program given");
    let mut vm_files: Vec<String> = Vec::new();

    // Read dir if dir path given
    if prog_arg.chars().last().unwrap() == '/' {
        let paths = fs::read_dir(prog_arg.to_string()).unwrap();
        for path in paths {
            let path_name = path.unwrap().path().to_str().unwrap().to_string();
            if path_name.contains(".vm") {
                vm_files.push(path_name.to_string());
            }
        }
    } else {
        vm_files.push(prog_arg.to_string());
    }
    if vm_files.len() == 0 {
        panic!("No .vm files found in {}", prog_arg);
    }

    let stack = Stack::new();
    let mut code_writer = CodeWriter::new();
    let mut vm_programs: Vec<Vec<String>> = Vec::new();
    let mut label_id = 0;

    for vm_file in vm_files.iter() {
        let vm_program = read_lines(vm_file.to_string());
        vm_programs.push(vm_program.clone());
        let mut parser = Parser::new(vm_program.clone());

        while parser.has_more_commands() {
            let current_command = parser.current_command();
            let command_type = parser.get_command_type();
            println!(
                "current command {} and type {:?}",
                current_command, command_type
            );
            match command_type {
                CommandType::C_Arithmetic => {
                    code_writer.write_arithmetic(current_command, &label_id.to_string());

                    // Another hacky thing :(
                    // TODO: Remove this crap
                    label_id += 1;
                }
                CommandType::C_Push | CommandType::C_Pop => {
                    code_writer.write_push_pop(
                        command_type,
                        &parser.get_arg1(),
                        &parser.get_arg2(),
                    );
                }
                _ => panic!("Unknown command type given!"),
            }
            println!("commands so far {:?}", code_writer.commands());
            parser.advance();
        }
    }

    let asm_filename = get_file_name(vm_files[0].to_string());
    write_to_file(&asm_filename, &code_writer.commands().join("\n"));
}
