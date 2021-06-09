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
}
