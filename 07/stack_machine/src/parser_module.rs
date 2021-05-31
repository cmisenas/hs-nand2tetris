use crate::stack::*;

/**
 * Handles the parsing of a single .vm file and encapsulates access to the input code.
 * It reads VM commands, prases them and provides convenient access to their components.
 * It also removes all white space and comments.
 */
pub struct Parser {
    index: usize,
    program: Vec<String>,
    current_command: String,
}

#[derive(Debug)]
pub enum CommandType {
    C_Arithmetic,
    C_Push,
    C_Pop,
    C_Label,
    C_Goto,
    C_If,
    C_Function,
    C_Return,
    C_Call,
}

impl Parser {
    // Opens the input file/stream and gets ready to parse it
    pub fn new(program: Vec<String>) -> Parser {
        Parser {
            index: 0,
            current_command: program.clone()[0].to_string(),
            program,
        }
    }

    pub fn current_command(&self) -> &String {
        &self.current_command
    }

    pub fn current_command_mut(&mut self) -> &mut String {
        &mut self.current_command
    }

    pub fn get_current_command_tokens(&self) -> Vec<&str> {
        self.current_command()
            .split(' ')
            .collect::<Vec<&str>>()
            .clone()
    }

    // Returns true if there are more commands in the input.
    // Otherwise returns false.
    pub fn has_more_commands(&self) -> bool {
        self.index < self.program.len()
    }

    // Reads the next command from the input and makes it the current command.
    // Should be called only if has_more_commands is true. Initially there is
    // no current command.
    pub fn advance(&mut self) {
        // Does a thing
        self.index += 1;
    }

    // Returns the type of the current VM command.
    // C_ARITHMETIC is returned for all the arithmetic commands.
    pub fn get_command_type(&self) -> CommandType {
        // Does a thing
        let first_token = self.get_current_command_tokens()[0];
        match first_token {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                CommandType::C_Arithmetic
            }
            "push" => CommandType::C_Push,
            "pop" => CommandType::C_Pop,
            _ => panic!("The rest of the commands will be handled on Project 8"),
        }
    }

    // Returns the first argument of the current command.
    // In the case of C_Arithmetic, the command itself (add, sub, etc) is returned.
    // Should not be called if the current command is C_Return.
    pub fn get_arg1(&self) -> String {
        match self.get_command_type() {
            CommandType::C_Return => panic!("No arg for CommandType::C_Return"),
            _ => self.get_current_command_tokens()[1].to_string(),
        }
    }

    // Returns the second argument of the current command. Should only be called
    // if the current command is C_Push, C_Pop, C_Function or C_Call.
    pub fn get_arg2(&self) -> String {
        match self.get_command_type() {
            CommandType::C_Push
            | CommandType::C_Pop
            | CommandType::C_Function
            | CommandType::C_Call => self.get_current_command_tokens()[2].to_string(),
            _ => panic!("No arg2 for command type"),
        }
    }
}
