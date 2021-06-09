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
                self.pop_val_sp();
                self.dec_sp();
                self.commands.push("D=M-D".to_string());
            }
            "add" => {
                self.pop_val_sp();
                self.dec_sp();
                self.commands.push("D=D+M".to_string());
            }
            "eq" | "lt" | "gt" => {
                let mut branch1 = label_id.to_string();
                branch1.push_str(".1");
                let mut branch2 = label_id.to_string();
                branch2.push_str(".2");
                let (eq_ptr, eq_label) = CodeWriter::get_label_ptr_pair(&branch1);
                let (d_eq_ptr, d_eq_label) = CodeWriter::get_label_ptr_pair(&branch2);

                self.pop_val_sp();
                self.dec_sp();
                self.commands.push("D=M-D".to_string());
                self.commands.push(eq_ptr.to_string());
                match command {
                    "eq" => self.commands.push("D;JEQ".to_string()),
                    "lt" => self.commands.push("D;JLT".to_string()),
                    // "gt" case
                    _ => self.commands.push("D;JGT".to_string()),
                }

                self.commands.push("D=0".to_string()); // Set to false
                self.commands.push(d_eq_ptr.to_string());
                self.commands.push("0;JMP".to_string());
                self.commands.push(eq_label.to_string());
                self.commands.push("D=-1".to_string()); // Set to true
                self.commands.push(d_eq_label.to_string());
            }
            "neg" => {
                self.pop_val_sp();
                self.commands.push("D=-D".to_string());
            }
            "not" => {
                self.pop_val_sp();
                self.commands.push("D=!D".to_string())
            }
            "and" => {
                self.pop_val_sp();
                self.dec_sp();
                self.commands.push("D=D&M".to_string());
            }
            "or" => {
                self.pop_val_sp();
                self.dec_sp();
                self.commands.push("D=D|M".to_string());
            }
            _ => panic!("Unknown command: {}", command),
        }
        self.set_m_to_sp();
        self.store_d();
    }

    fn get_label_ptr_pair(label_id: &str) -> (String, String) {
        let mut name = "LABEL_".to_string();
        name.push_str(label_id);
        let mut ptr = "@".to_string();
        ptr.push_str(&name);
        let mut label = "()".to_string();
        label.insert_str(1, &name);
        (ptr, label)
    }

    fn set_segment_index_val(&mut self, segment: &str, offset: &str) {
        let mut addr = offset.parse::<usize>().unwrap();
        let segment_addr = match segment {
            "local" => "@LCL".to_string(),
            "argument" => "@ARG".to_string(),
            "this" => "@THIS".to_string(),
            "that" => "@THAT".to_string(),
            _ => panic!("Unknown segment: {}", segment),
        };

        let mut offset_addr = "@".to_string();
        offset_addr.push_str(offset);

        // Set @segment to @segment[index] addr
        self.commands.push(offset_addr.to_string());
        self.commands.push("D=A".to_string());
        self.commands.push(segment_addr.to_string());
        self.commands.push("D=M+D".to_string());

        // Use @R13 which is a general-purpose register to
        // temporarily set the address of @segment[index]
        self.commands.push("@R13".to_string());
        self.commands.push("M=D".to_string());

        // Pop stack
        self.pop_val_sp();

        // Set @segment to D
        self.commands.push("@R13".to_string());
        self.commands.push("A=M".to_string());
        self.commands.push("M=D".to_string());
    }

    // pop segment index - Pop the top stack value and store it in segment[index].
    fn pop_val(&mut self, segment: &str, val: &str) {
        match segment {
            "constant" => {
                self.pop_val_sp();
            }
            "local" | "argument" | "this" | "that" => {
                self.set_segment_index_val(segment, val);
            }
            "temp" => {
                let mut segment_addr = "@R".to_string();
                // Starts from @R5-@R12
                let offset_val = val.parse::<usize>().unwrap() + 5;
                segment_addr.push_str(&offset_val.to_string());
                if offset_val > 12 || offset_val < 5 {
                    panic!("Invalid temp address {}", offset_val);
                }

                // Pop stack
                self.pop_val_sp();

                // Set @segment to D
                self.commands.push(segment_addr.to_string());
                self.commands.push("M=D".to_string());
            }
            "static" => {
                let mut segment_addr = "@".to_string();
                // Static memory segment is from RAM[16]-RAM[255]
                let offset_val = val.parse::<usize>().unwrap() + 16;
                segment_addr.push_str(&offset_val.to_string());
                if offset_val > 255 || offset_val < 16 {
                    panic!("Invalid static address {}", offset_val);
                }

                // Pop stack
                self.pop_val_sp();

                // Set @segment to D
                self.commands.push(segment_addr.to_string());
                self.commands.push("M=D".to_string());
            }
            "pointer" => {
                let mut segment_addr = "@".to_string();
                // Pointer is mapped to locations 3-4 (also called THIS and THAT)
                let offset_val = val.parse::<usize>().unwrap() + 3;
                segment_addr.push_str(&offset_val.to_string());
                if offset_val > 4 || offset_val < 3 {
                    panic!("Invalid pointer address {}", offset_val);
                }

                // Pop stack
                self.pop_val_sp();

                // Set @segment to D
                self.commands.push(segment_addr.to_string());
                self.commands.push("M=D".to_string());
            }
            _ => panic!("Unknown segment: {}", segment),
        }
    }

    fn get_segment_index_val(&mut self, segment: &str, offset: &str) {
        let mut addr = offset.parse::<usize>().unwrap();
        let segment_addr = match segment {
            "local" => "@LCL".to_string(),
            "argument" => "@ARG".to_string(),
            "this" => "@THIS".to_string(),
            "that" => "@THAT".to_string(),
            _ => panic!("Unknown segment: {}", segment),
        };

        let mut offset_addr = "@".to_string();
        offset_addr.push_str(offset);

        // Set @segment to @segment[index] addr
        self.commands.push(offset_addr.to_string());
        self.commands.push("D=A".to_string());
        self.commands.push(segment_addr.to_string());
        self.commands.push("A=M+D".to_string());
        self.commands.push("D=M".to_string());

        // Push stack
        self.set_m_to_sp();
        self.store_d();
    }

    // push segment index - Push the value of segment[index] onto the stack.
    fn push_val(&mut self, segment: &str, val: &str) {
        let mut addr = "@".to_string();
        addr.push_str(val);
        match segment {
            "constant" => {
                self.commands.push(addr.to_string());
                self.commands.push("D=A".to_string());
                self.set_m_to_sp();
                self.store_d();
            }
            "local" | "argument" | "this" | "that" => {
                self.get_segment_index_val(segment, val);
            }
            "temp" => {
                let mut segment_addr = "@R".to_string();
                // Starts from @R5-@R12
                let offset_val = val.parse::<usize>().unwrap() + 5;
                segment_addr.push_str(&offset_val.to_string());
                if offset_val > 12 || offset_val < 5 {
                    panic!("Invalid temp address {}", offset_val);
                }

                // Set A to temp address
                self.commands.push(segment_addr.to_string());
                self.commands.push("D=M".to_string());

                // Push stack
                self.set_m_to_sp();
                self.store_d();
            }
            "static" => {
                let mut segment_addr = "@".to_string();
                // Static memory segment is from RAM[16]-RAM[255]
                let offset_val = val.parse::<usize>().unwrap() + 16;
                segment_addr.push_str(&offset_val.to_string());
                if offset_val > 255 || offset_val < 16 {
                    panic!("Invalid temp address {}", offset_val);
                }

                // Set A to temp address
                self.commands.push(segment_addr.to_string());
                self.commands.push("D=M".to_string());

                // Push stack
                self.set_m_to_sp();
                self.store_d();
            }
            "pointer" => {
                let mut segment_addr = "@".to_string();
                // Pointer is mapped to locations 3-4 (also called THIS and THAT)
                let offset_val = val.parse::<usize>().unwrap() + 3;
                segment_addr.push_str(&offset_val.to_string());
                if offset_val > 4 || offset_val < 3 {
                    panic!("Invalid pointer address {}", offset_val);
                }

                // Set A to temp address
                self.commands.push(segment_addr.to_string());
                self.commands.push("D=M".to_string());

                // Push stack
                self.set_m_to_sp();
                self.store_d();
            }
            _ => panic!("Unknown segment: {}", segment),
        }
    }

    // Decrements SP
    // Sets D to SP
    fn pop_val_sp(&mut self) {
        self.dec_sp();
        self.commands.push("D=M".to_string());
    }

    // Convenience function to store the current value to SP
    fn set_m_to_sp(&mut self) {
        self.commands.push("@SP".to_string());
        self.commands.push("A=M".to_string());
    }

    // Stores a value to A address
    // Increments SP
    fn store_d(&mut self) {
        self.commands.push("M=D".to_string());
        self.inc_sp();
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
