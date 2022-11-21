use std::io::Write;

fn main() {
    // Test your bf programs at https://copy.sh/brainfuck/

    // print "!"
    // let str_program = "+++++ +++++ +++++ +++++ +++++ +++++ +++.";

    // print "!" w/ minuses
    // let str_program = "+++++ - +++++ - +++++ +++++ +++++ +++++ +++++ +-.";

    // print "HI" with two slots
    // let str_program = "
    //     ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++.
    //     >
    //     ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +++.
    // ";

    // print "HI" with back and forward movement
    // let str_program = "
    //     ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +++
    //     >
    //     ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++.
    //     < .
    // ";

    // Prints "54321"
    // let str_program = "
    //     ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +++
    //     > +++++
    //     [<.->-]
    // ";

    // Prints "555444333222111" w/ nested loops
    // let str_program = "
    //     ++++++++++ ++++++++++ ++++++++++ ++++++++++ ++++++++++ +++
    //     > +++++
    //     [
    //     > +++
    //     [<<.>>-]
    //     <<-
    //     >-
    //     ]
    // ";

    // Syntax errors (these should fail)
    // let str_program = "]";
    // let str_program = "++]";
    // let str_program = "[]]";

    // Hello World
    let str_program = "
        >++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
        +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
        ]<+.
    ";

    // Read and print
    // let str_program = ",.,.";


    let program: Vec<char> = str_program.chars().collect();
    let mut counter = 0;

    let mut memory: Vec<u32> = vec![0; 30000];
    let mut pointer = 0;

    let mut loop_starts: Vec<usize> = Vec::new();

    while counter < str_program.len() {
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
