use crate::parser_module::*;

/**
 * Translates VM commands into Hack assembly code.
 */
pub struct CodeWriter {
    index: usize,
    commands: Vec<String>,
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter {
            index: 0,
            commands: Vec::new(),
        }
    }

    pub fn commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn commands_mut(&mut self) -> &mut Vec<String> {
        &mut self.commands
    }

    // Informs the code writer that the translation
    // of a new VM file is started
    pub fn set_file_name(&self, file_name: String) {
        // Does a thing
    }

    // Writes the assembly code that is the translation
    // of the given arithmetic command.
    pub fn write_arithmetic(&mut self, command: &str, label_id: &str) {
        // Responsible for setting comp and jump bits for C_Instruction
        // -1 = True
        // 0 = False
        match command {
            "sub" => {
                self.commands.push("D=D-A".to_string());
            }
            "add" => {
                self.commands.push("D=D+A".to_string());
            }
            "eq" | "lt" | "gt" => {
                let mut eq_name = "LABEL_".to_string();
                eq_name.push_str(label_id);
                let mut eq_ptr = "@".to_string();
                eq_ptr.push_str(&eq_name);
                let mut eq_label = "()".to_string();
                eq_label.insert_str(1, &eq_name);

                let mut d_eq_name = "D_LABEL_".to_string();
                d_eq_name.push_str(label_id);
                let mut d_eq_ptr = "@".to_string();
                d_eq_ptr.push_str(&d_eq_name);
                let mut d_eq_label = "()".to_string();
                d_eq_label.insert_str(1, &d_eq_name);

                self.commands.push("D=D-A".to_string());
                self.commands.push(eq_ptr.to_string());
                match command {
                    "eq" => self.commands.push("D;JEQ".to_string()),
                    "lt" => self.commands.push("D;JLT".to_string()),
                    "gt" => self.commands.push("D;JGT".to_string()),
                    _ => panic!("Poopsie"),
                }
                self.commands.push("D=0".to_string());
                self.commands.push(d_eq_ptr.to_string());
                self.commands.push("0;JMP".to_string());
                self.commands.push(eq_label.to_string());
                self.commands.push("D=-1".to_string());
                self.commands.push(d_eq_label.to_string());
            }
            "neg" => self.commands.push("D=-D".to_string()),
            "and" => self.commands.push("D=D&A".to_string()),
            "or" => self.commands.push("D=D|A".to_string()),
            "not" => self.commands.push("D=!A".to_string()),
            _ => panic!("Unknown command: {}", command),
        }
        // Push results to the top of the stack
        self.commands.push("@SP".to_string());
        self.commands.push("A=M".to_string());
        self.commands.push("M=D".to_string());
        // Since these op results are a push to the top of the stack
        // we need to increment SP
        self.inc_sp();
    }

    fn pop_val(&mut self, segment: &str, val: &str) {
        let mut addr = "@".to_string();
        match segment {
            "constant" => {
                self.dec_sp();
                addr.push_str("SP");
                self.commands.push(addr.to_string());
                self.commands.push("A=M".to_string());
                self.commands.push("M=D".to_string());
            }
            _ => panic!("Unknown segment: {}", segment),
        }
    }

    fn push_val(&mut self, segment: &str, val: &str) {
        let mut addr = "@".to_string();
        match segment {
            "constant" => {
                addr.push_str(val);
                self.commands.push(addr.to_string());
                self.commands.push("D=A".to_string());
                // No need to increment SP since we did not write anything to memory segments
            }
            _ => panic!("Unknown segment: {}", segment),
        }
    }

    fn inc_sp(&mut self) {
        self.commands.push("@SP".to_string());
        self.commands.push("AM=M+1".to_string());
    }

    fn dec_sp(&mut self) {
        self.commands.push("@SP".to_string());
        self.commands.push("AM=M-1".to_string());
    }

    // Writes the assembly code that is the translation
    // of the given command, where command is either C_Push or C_Pop
    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, val: &str) {
        match command {
            CommandType::C_Push => self.push_val(segment, val),
            CommandType::C_Pop => self.pop_val(segment, val),
            _ => panic!("Unknown command type: {:?}", command),
        }
    }

    // Closes the output file
    pub fn close(&self, command: String) {
        // Does a thing
    }
}
