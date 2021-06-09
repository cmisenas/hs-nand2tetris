use crate::utils::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Taken from https://rust-lang-nursery.github.io/rust-cookbook/mem/global_static.html
lazy_static! {
    #[derive(Debug)]
    pub static ref PREDEF_SYMBOLS: HashMap<String, String> = {
        init_bit_spec(read_lines("./src/predefined_symbols.txt"))
    };
}

#[derive(Debug)]
pub struct SymbolTable {
    pub var_index: i16,
    pub table: HashMap<String, String>,
}

/**
 * 3 sources of symbols:
 * - Predefined symbols
 *   - Predefined labels that map to a specific RAM address
 * - Label symbols
 *   - Refer to the instruction memory location holding the next command in the program
 *     Note: (LABELS) are not counted in the address
 * - Variable symbols
 *   - Any symbol not predefined/label. Mapped to consecutive memory locations as they
 *     are first encountered, starting at RAM address 16 (0x0010)
 */

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            var_index: 16,
            table: HashMap::new(),
        }
    }

    // Adds the pair (symbol, address) to the table
    pub fn add_entry(&mut self, symbol: &str, address: String) {
        // If variable, increase self.var_index
        self.table.insert(symbol.to_string(), address.to_string());
    }

    // Adds the symbol to the table and increases the var_index by itself
    pub fn add_var_entry(&mut self, symbol: &str) {
        self.add_entry(symbol, self.var_index.to_string());
        self.var_index += 1;
    }

    // Returns true if the symbol table contains the given symbol.
    // Otherwise, false.
    pub fn contains(&self, symbol: &str) -> bool {
        PREDEF_SYMBOLS.contains_key(symbol) || self.table.contains_key(symbol)
    }

    // Returns the address associated with the symbol.
    pub fn get_address(&self, symbol: &str) -> String {
        if let Some(val) = PREDEF_SYMBOLS.get(symbol) {
            val.to_string()
        } else if let Some(val) = self.table.get(symbol) {
            val.to_string()
        } else {
            panic!("Symbol {} not found!", symbol);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_functionality() {
        let mut symbol_table = SymbolTable::new();
        assert_eq!(symbol_table.contains("@THIS"), true);
        assert_eq!(symbol_table.contains("@PANDABEAR"), false);

        symbol_table.add_entry("@PANDABEAR", "5".to_string());
        assert_eq!(symbol_table.contains("@PANDABEAR"), true);

        assert_eq!(symbol_table.get_address("@THIS"), "3");
        assert_eq!(symbol_table.get_address("@PANDABEAR"), "5");
    }
}
