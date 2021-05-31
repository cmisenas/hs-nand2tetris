use crate::parser_module::*;

/**
 * Translates VM commands into Hack assembly code.
 */
pub struct CodeWriter {
    index: usize,
}

impl CodeWriter {
    pub fn new() -> CodeWriter {
        CodeWriter { index: 0 }
    }

    // Informs the code writer that the translation
    // of a new VM file is started
    pub fn set_file_name(&self, file_name: String) {
        // Does a thing
    }

    // Writes the assembly code that is the translation
    // of the given arithmetic command.
    pub fn write_arithmetic(&self, command: String) {
        // Does a thing
    }

    // Writes the assembly code that is the translation
    // of the given command, where command is either C_Push or C_Pop
    pub fn write_push_pop(&self, command: CommandType, segment: String, index: i16) {
        // Does a thing
    }

    // Closes the output file
    pub fn close(&self, command: String) {
        // Does a thing
    }
}
