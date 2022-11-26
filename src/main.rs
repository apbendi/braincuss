use std::io::{Read, Write};
mod vm;

// Test your bf programs at https://copy.sh/brainfuck/
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_arg: String;

    if args.len() == 2 {
        input_arg = String::from("");
    } else if args.len() == 3 {
        input_arg = args[2].clone(); // way to do this w/o cloning?
    } else {
        println!("Usage: braincuss <PROGRAM_FILE> [INPUT");
        std::process::exit(0);
    }

    let file_path = &args[1];
    let program = read_source(file_path);

    let program_input: Vec<char> = input_arg.chars().collect();
    let mut input_pointer: usize = 0;

    let mut vm = vm::Machine::with_memory_size_and_program(30_000, &program);

    let mut loop_starts: Vec<usize> = Vec::new();

    while !vm.end_of_program() {
        match vm.instruction() {
            '+' => vm.increment_memory(),
            '-' => vm.decrement_memory(),
            '>' => vm.advance_pointer(),
            '<' => vm.reverse_pointer(),
            '.' => print_as_char(vm.read_memory()),
            ',' => {
                let byte: u8;

                if input_pointer == program_input.len() {
                    byte = 0;
                } else {
                    byte = program_input[input_pointer] as u8;
                    input_pointer += 1;
                }

                vm.write_memory(byte);
            },
            '[' => {
                if vm.read_memory() == 0 {
                    let mut inner_loop_counter = 0;
                    // We've hit a loop but the pointer is already zero, so we don't
                    // execute the loop even once—skip straight to the close of the loop.
                    loop {
                        vm.advance_pc();
                        if vm.instruction() == '[' {
                            // we entered an inner loop
                            inner_loop_counter += 1;
                        } else if vm.instruction() == ']' {
                            if inner_loop_counter > 0 {
                                // we're exiting an inner loop
                                inner_loop_counter -= 1;
                            } else {
                                // we're exiting the initial loop
                                break;
                            }
                        } else if vm.end_of_program() {
                            // bracket was never closed, should error?
                            break;
                        }
                    }
                } else {
                    loop_starts.push(vm.pc());
                }
            },
            ']' => {
                let start = loop_starts
                                .last()
                                .expect("Syntax error: Unexpected closing bracket");

                if vm.read_memory() == 0 {
                    loop_starts.pop();
                } else if vm.read_memory() != 0 {
                    vm.jump(*start);

                }
            }
            _ => (),
        }

        vm.advance_pc();
    }

    print!("\n");
}

fn read_source(path: &String) -> String {
    match std::fs::File::open(path) {
        Ok (mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        },
        Err(error) => {
            println!("Error opening file {}: {}", path, error);
            std::process::exit(1);
        }
    }
}

fn print_as_char(mem: u8) {
    print!("{}", mem as char);

    // Flush the buffer
    std::io::stdout()
        .flush()
        .expect("Output Failed");
}
