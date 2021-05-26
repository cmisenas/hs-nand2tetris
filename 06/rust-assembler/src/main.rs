use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*; // Needed for write_all
use std::io::{self, BufRead};
use std::path::Path;

// Taken from https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html
lazy_static! {
    #[derive(Debug)]
    static ref COMP_BITS: HashMap<String, String> = {
        init_bit_spec(read_lines("./src/comp_bits.txt"))
    };

    #[derive(Debug)]
    static ref DEST_BITS: HashMap<String, String> = {
        init_bit_spec(read_lines("./src/dest_bits.txt"))
    };

    #[derive(Debug)]
    static ref JUMP_BITS: HashMap<String, String> = {
        init_bit_spec(read_lines("./src/jump_bits.txt"))
    };
}

fn strip_comments(statement: String) -> String {
    statement.split("//").collect::<Vec<&str>>()[0]
        .trim()
        .to_string()
}

fn init_bit_spec(specs: Vec<String>) -> HashMap<String, String> {
    // There must be a better way??
    let _bits: Vec<(String, String)> = specs
        .iter()
        .cloned()
        .map(|bit| {
            let _bit: Vec<&str> = bit.split_ascii_whitespace().collect::<Vec<&str>>();
            (_bit[0].to_string(), _bit[1].to_string())
        })
        .collect();
    _bits.iter().cloned().collect::<HashMap<String, String>>()
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

fn generate_c_ins(statement: &String) -> String {
    let jump_parts: Vec<&str> = statement.split(|c| c == ';').collect();
    let comp_parts: Vec<&str> = jump_parts[0].split(|c| c == '=').collect();
    let comp: &str = match comp_parts.len() {
        2 => comp_parts[1],
        _ => comp_parts[0],
    };
    let dest: &str = match comp_parts.len() {
        2 => comp_parts[0],
        _ => "null",
    };
    let jump: &str = match jump_parts.len() {
        2 => jump_parts[1],
        _ => "null",
    };
    let mut c_ins = "111".to_string();
    c_ins.push_str(COMP_BITS.get(comp).unwrap());
    c_ins.push_str(DEST_BITS.get(dest).unwrap());
    c_ins.push_str(JUMP_BITS.get(jump).unwrap());
    c_ins
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
            result.push(generate_c_ins(statement));
        }
    }
    write_to_file(&hack_filename, &result.join("\n"));
}

// Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
fn write_to_file(filename: &str, content: &str) {
    let path = Path::new(filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `content` string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
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
