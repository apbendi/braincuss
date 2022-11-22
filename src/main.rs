use std::io::{Write, Read};

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

    let mut memory: Vec<u32> = vec![0; 30000];
    let mut pointer = 0;

    let mut loop_starts: Vec<usize> = Vec::new();

    while counter < program.len() {
        if program[counter] == '.' {
            print_memory(memory[pointer]);
        } else if program[counter] == '+' {
            memory[pointer] += 1;
        } else if program[counter] == '-' {
            memory[pointer] -= 1;
        } else if program[counter] == '>' {
            pointer += 1;
        } else if program[counter] == '<' {
            pointer -= 1;
        } else if program[counter] == '[' {
            loop_starts.push(counter);
        } else if program[counter] == ']' {
            let start = loop_starts
                                .last()
                                .expect("Syntax error: Unexpected closing bracket");

            if memory[pointer] == 0 {
                loop_starts.pop();
            } else if memory[pointer] != 0 {
                counter = *start;
            }
        } else if program[counter] == ',' {
            memory[pointer] = read_char();
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

fn print_memory(mem: u32) {
    unsafe {
        let c: char = char::from_u32_unchecked(mem);
        print!("{}", c);
    }

    // Flush the buffer
    std::io::stdout()
        .flush()
        .expect("Output Failed");
}

fn read_char() -> u32 {
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
    first as u32
}
