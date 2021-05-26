use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/**
 * 4 main tasks of an assembler:
 * - Parsing
 * - Code generation
 * - Symbols handling *
 * - Final assembly
 */

// TODO:
// x - Initialize machine language specification
// Get <program file name>.asm
// Generate <program file name>.hack
// Create a parser for program

fn symbolless_assembly() {
    let args: Vec<String> = env::args().collect();
    let prog_file = args[1].parse::<String>().expect("No program given");
    let program = read_lines(prog_file);
    println!("{:?}", program);
}

fn init_bits() {
    let comp_bits: HashMap<String, String> = init_bit_spec(read_lines("./src/comp_bits.txt"));
    let dest_bits: HashMap<String, String> = init_bit_spec(read_lines("./src/dest_bits.txt"));
    let jump_bits: HashMap<String, String> = init_bit_spec(read_lines("./src/jump_bits.txt"));
    println!("{:?}", comp_bits);
    println!("{:?}", dest_bits);
    println!("{:?}", jump_bits);
}

fn init_bit_spec(specs: Vec<String>) -> HashMap<String, String> {
    // There must be a better way??
    let _bits: Vec<(String, String)> = specs
        .iter()
        .cloned()
        .map(|comp_bit| {
            let _bit: Vec<&str> = comp_bit.split_ascii_whitespace().collect::<Vec<&str>>();
            (_bit[0].to_string(), _bit[1].to_string())
        })
        .collect();
    _bits.iter().cloned().collect::<HashMap<String, String>>()
}

fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not find file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<String>().unwrap())
        .collect()
}

fn main() {
    println!("Hello, world!");
    init_bits();
    symbolless_assembly();
}
