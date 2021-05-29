/**
 * Handles the parsing of a single .vm file and encapsulates access to the input code.
 * It reads VM commands, prases them and provides convenient access to their components.
 * It also removes all white space and comments.
 */
struct Parser {
    pub index: usize,
    pub program: Vec<String>,
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
        Parser { index: 0, program }
    }

    // Returns true if there are more commands in the input.
    // Otherwise returns false.
    pub fn has_more_commands(&self) -> bool {
        true
    }

    // Reads the next command from the input and makes it the current command.
    // Should be called only if has_more_commands is true. Initially there is
    // no current command.
    pub fn advance(&self) {
        // Does a thing
    }

    // Returns the type of the current VM command.
    // C_ARITHMETIC is returned for all the arithmetic commands.
    pub fn get_command_type(&self) -> CommandType {
        // Does a thing
        CommandType::C_Arithmetic
    }

    // Returns the first argument of the current command.
    // In the case of C_Arithmetic, the command itself (add, sub, etc) is returned.
    // Should not be called if the current command is C_Return.
    pub fn get_arg1(&self) {
        // Does a thing
    }

    // Returns the second argument of the current command. Should only be called
    // if the current command is C_Push, C_Pop, C_Function or C_Call.
    pub fn get_arg2(&self) {
        // Does a thing
    }
}
