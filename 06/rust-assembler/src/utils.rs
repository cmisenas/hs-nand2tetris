use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*; // Needed for write_all
use std::io::{self, BufRead};
use std::path::Path;

// Taken from https://doc.rust-lang.org/rust-by-example/std_misc/file/create.html
pub fn write_to_file(filename: &str, content: &str) {
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

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Could not find file");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line").parse::<String>().unwrap())
        .collect()
}

pub fn to_binary(x: i16) -> String {
    // NOTE: This won't work if a negative value is passed?
    format!("{:016b}", x)
}

pub fn init_bit_spec(specs: Vec<String>) -> HashMap<String, String> {
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
