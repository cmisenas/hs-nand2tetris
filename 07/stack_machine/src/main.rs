mod code_writer_module;
mod parser_module;
mod stack;

/**
 * From the book: The main program should construct:
 * - a Parser to parse the VM input file
 * - a CodeWriter to generate code into the corresponding output file
 * It should then march through the VM commands in the input file and
 * generate assembly code for each one of them.
 *
 * If the program's argument is a directory name rather than a file name,
 * the main program should process all the .vm files in this directory.
 * In doing so, it should use a separate Parser for handling each input file
 * and a single CodeWriter for handling the output.
 *
 */
fn main() {
    println!("Virtual machine 1: Stack arithmetic and memory access");
}
