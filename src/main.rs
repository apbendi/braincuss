use std::io::{Read, Write};
mod vm;

// Test your bf programs at https://copy.sh/brainfuck/
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args {:?}", args);

    if args.len() != 2 {
        println!("Usage: braincuss <PROGRAM_FILE>");
        std::process::exit(0);
    }

    let file_path = &args[1];
    let program = read_source(file_path);

    let program: Vec<char> = program.chars().collect();
    let mut counter = 0;

    let mut vm = vm::Machine::with_memory_size(30_000);

    let mut loop_starts: Vec<usize> = Vec::new();

    while counter < program.len() {
        if program[counter] == '.' {
            print_as_char(vm.read_memory());
        } else if program[counter] == '+' {
           vm.increment_memory();
        } else if program[counter] == '-' {
            vm.decrement_memory();
        } else if program[counter] == '>' {
            vm.advance_pointer();
        } else if program[counter] == '<' {
            vm.reverse_pointer();
        } else if program[counter] == '[' {
            if vm.read_memory() == 0 {
                let mut inner_loop_counter = 0;
                // We've hit a loop but the pointer is already zero, so we don't
                // execute the loop even once—skip straight to the close of the loop.
                loop {
                    counter += 1;
                    if program[counter] == '[' {
                        // we entered an inner loop
                        inner_loop_counter += 1;
                    } else if program[counter] == ']' {
                        if inner_loop_counter > 0 {
                            // we're exiting an inner loop
                            inner_loop_counter -= 1;
                        } else {
                            // we're exiting the initial loop
                            break;
                        }
                    } else if counter == program.len() - 1 {
                        // bracket was never closed, should error?
                        break;
                    }
                }
            } else {
                loop_starts.push(counter);
            }
        } else if program[counter] == ']' {
            let start = loop_starts
                                .last()
                                .expect("Syntax error: Unexpected closing bracket");

            if vm.read_memory() == 0 {
                loop_starts.pop();
            } else if vm.read_memory() != 0 {
                counter = *start;

            }
        } else if program[counter] == ',' {
            vm.write_memory(read_char());
        }

        counter += 1;
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
            println!("Error opening file {}", error);
            std::process::exit(1);
        }
    }
}

fn print_as_char(mem: u8) {
    unsafe {
        let c: char = char::from_u32_unchecked(mem as u32);
        print!("{}", c);
    }

    // Flush the buffer
    std::io::stdout()
        .flush()
        .expect("Output Failed");
}

fn read_char() -> u8 {
    print!("\n"); // Newline before taking input
    let mut input_string = String::new();
    let stdin = std::io::stdin();
    stdin
        .read_line(&mut input_string)
        .expect("Read from stdin failed");

    let first = input_string
                        .chars()
                        .next()
                        .expect("Input Failed");
    first as u8
}
