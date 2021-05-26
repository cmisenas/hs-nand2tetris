use crate::utils::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Taken from https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html
lazy_static! {
    #[derive(Debug)]
    pub static ref COMP_BITS: HashMap<String, String> = {
        init_bit_spec(read_lines("./src/comp_bits.txt"))
    };

    #[derive(Debug)]
    pub static ref DEST_BITS: HashMap<String, String> = {
        init_bit_spec(read_lines("./src/dest_bits.txt"))
    };

    #[derive(Debug)]
    pub static ref JUMP_BITS: HashMap<String, String> = {
        init_bit_spec(read_lines("./src/jump_bits.txt"))
    };
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

pub fn generate_c_ins(statement: &str) -> String {
    let jump_parts: Vec<&str> = statement.split(|c| c == ';').collect();
    let comp_parts: Vec<&str> = jump_parts[0].split(|c| c == '=').collect();
    let comp_bits: &str = match comp_parts.len() {
        2 => comp_parts[1],
        _ => comp_parts[0],
    };
    let dest_bits: &str = match comp_parts.len() {
        2 => comp_parts[0],
        _ => "null",
    };
    let jump_bits: &str = match jump_parts.len() {
        2 => jump_parts[1],
        _ => "null",
    };
    let mut c_ins = "111".to_string();
    c_ins.push_str(&comp(comp_bits));
    c_ins.push_str(&dest(dest_bits));
    c_ins.push_str(&jump(jump_bits));
    c_ins
}

pub fn comp(mnemonic: &str) -> String {
    COMP_BITS.get(mnemonic).unwrap().to_string()
}

pub fn dest(mnemonic: &str) -> String {
    DEST_BITS.get(mnemonic).unwrap().to_string()
}

pub fn jump(mnemonic: &str) -> String {
    JUMP_BITS.get(mnemonic).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comp_works() {
        assert_eq!(comp("D+1"), "0011111");
        assert_eq!(comp("0"), "0101010");
    }

    #[test]
    fn dest_works() {
        assert_eq!(dest("null"), "000");
        assert_eq!(dest("MD"), "011");
    }

    #[test]
    fn jump_works() {
        assert_eq!(jump("null"), "000");
        assert_eq!(jump("JGE"), "011");
    }

    #[test]
    fn generate_c_ins_works() {
        assert_eq!(generate_c_ins("D=M"), "1111110000010000");
        assert_eq!(generate_c_ins("D;JLE"), "1110001100000110");
    }
}
