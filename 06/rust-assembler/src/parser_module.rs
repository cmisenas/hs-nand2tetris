use crate::code_module::*;

#[derive(Debug)]
pub struct Parser {
    pub index: usize,
    pub program: Vec<String>,
    pub comp_bits: String,
    pub dest_bits: String,
    pub jump_bits: String,
}

pub enum CommandType {
    // - ACommand for @Xxx where Xxx is either a symbol or a decimal number
    ACommand,
    // - CCommand for dest=comp;jump
    CCommand,
    // - LCommand (actually, pseudo-command) for (Xxx) where Xxx is a symbol
    LCommand,
}

impl Parser {
    // Receives the program as a vec of str
    pub fn new(program: Vec<String>) -> Parser {
        Parser {
            index: 0,
            program,
            comp_bits: "".to_string(),
            dest_bits: "".to_string(),
            jump_bits: "".to_string(),
        }
    }

    // Returns true if there are more commands in the input
    pub fn has_more_commands(&self) -> bool {
        self.index < self.program.len()
    }

    // Reads the next command from the input and makes it the current command.
    // Should only be called only if has_more_commands is true. Initially
    // there is no current command.
    pub fn advance(&mut self) {
        self.index += 1;
        // On each advance, cache each part of the command if CCommand type
        if self.has_more_commands() {
            self.init_c_command();
        }
    }

    pub fn init(&mut self) {
        self.init_c_command();
    }

    fn init_c_command(&mut self) {
        match self.command_type() {
            CommandType::CCommand => {
                let jump_parts: Vec<String> = self
                    .current_command()
                    .split(';')
                    .map(|c| c.to_string())
                    .collect();
                let comp_parts: Vec<String> =
                    jump_parts[0].split('=').map(|c| c.to_string()).collect();
                self.comp_bits = match comp_parts.len() {
                    2 => comp_parts[1].to_string(),
                    _ => comp_parts[0].to_string(),
                };
                self.dest_bits = match comp_parts.len() {
                    2 => comp_parts[0].to_string(),
                    _ => "null".to_string(),
                };
                self.jump_bits = match jump_parts.len() {
                    2 => jump_parts[1].to_string(),
                    _ => "null".to_string(),
                }
                .to_string();
            }
            _ => {
                self.comp_bits = "".to_string();
                self.dest_bits = "".to_string();
                self.jump_bits = "".to_string();
            }
        }
    }

    // Returns the type of the current command.
    pub fn command_type(&self) -> CommandType {
        if self.program[self.index].starts_with('@') {
            return CommandType::ACommand;
        } else if self.program[self.index].starts_with('(') {
            return CommandType::LCommand;
        }
        CommandType::CCommand
    }

    // Returns the symbol or decimal Xxx of the current command @Xxx or (Xxx).
    // Should be called only when command_type is ACommand or LCommand
    pub fn symbol(&self) -> i16 {
        // Assumption here is that a valid .asm file has been passed
        match self.command_type() {
            CommandType::ACommand => self
                .current_command()
                .strip_prefix('@')
                .unwrap()
                .parse::<i16>()
                .unwrap(),
            CommandType::LCommand => self
                .current_command()
                .trim_matches(|c| c == '(' || c == ')')
                .parse::<i16>()
                .unwrap(),
            _ => panic!("Not an ACommand or LCommand"),
        }
    }

    fn current_command(&self) -> &str {
        &self.program[self.index]
    }

    // Returns the dest mnemonic in the current C-command (8 possibilities).
    // Should be called only when command_type is CCommand
    pub fn dest(&self) -> String {
        match self.command_type() {
            CommandType::CCommand => dest(&self.dest_bits),
            _ => panic!("Not a CCommand"),
        }
    }

    // Returns the comp mnemonic in the current C-command (28 possibilities).
    // Should be called only when command_type is CCommand
    pub fn comp(&self) -> String {
        match self.command_type() {
            CommandType::CCommand => comp(&self.comp_bits),
            _ => panic!("Not a CCommand"),
        }
    }

    // Returns the jump mnemonic in the current C-command (8 possibilities).
    // Should be called only when command_type is CCommand
    pub fn jump(&self) -> String {
        match self.command_type() {
            CommandType::CCommand => jump(&self.jump_bits),
            _ => panic!("Not a CCommand"),
        }
    }

    fn to_binary(&self, x: i16) -> String {
        // Where x is either a non-negative decimal number
        // or a symbol referring to such number.
        // NOTE: This won't work if a negative value is passed?
        format!("{:016b}", x)
    }

    // I added these 2 for convenience but not sure if this is the way to go
    pub fn generate_a_ins(&mut self) -> String {
        // Return 0xxx xxxx xxxx xxxx
        self.to_binary(self.symbol())
    }

    pub fn generate_c_ins(&mut self) -> String {
        let mut c_ins = "111".to_string();
        c_ins.push_str(&self.comp());
        c_ins.push_str(&self.dest());
        c_ins.push_str(&self.jump());
        c_ins
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_generate_c_ins_works() {
        let test_program = vec!["D=M".to_string(), "D;JLE".to_string()];
        let mut parser = Parser::new(test_program.clone());
        parser.init();
        assert_eq!(parser.index, 0);
        assert_eq!(parser.current_command(), "D=M");
        assert_eq!(parser.has_more_commands(), true);
        assert_eq!(parser.generate_c_ins(), "1111110000010000");
        parser.advance();
        assert_eq!(parser.has_more_commands(), true);
        assert_eq!(parser.generate_c_ins(), "1110001100000110");
        parser.advance();
        assert_eq!(parser.has_more_commands(), false);
    }
}
