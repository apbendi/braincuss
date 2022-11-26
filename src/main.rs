use std::io::{Read, Write};
mod vm;
mod interpreter;

// Test your bf programs at https://copy.sh/brainfuck/
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let input_arg: String;

    match args.len() {
        2 => input_arg = String::from(""),
        3 => input_arg = args[2].clone(), // way to do this w/o cloning?
        _ => {
            println!("Usage: braincuss <PROGRAM_FILE> [INPUT");
            std::process::exit(0);
        }
    }

    let file_path = &args[1];
    let program = read_source(file_path);

    interpreter::run(&program, &input_arg, &print_as_char);

    // trailing newline for terminal
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
