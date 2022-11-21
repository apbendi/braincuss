fn main() {
    // print "!"
    // let str_program = "+++++ +++++ +++++ +++++ +++++ +++++ +++.";

    // print "!" w/ minuses
    let str_program = "+++++ - +++++ - +++++ +++++ +++++ +++++ +++++ +-.";

    let program: Vec<char> = str_program.chars().collect();
    let mut counter = 0;

    let mut memory: Vec<u32> = vec![0; 30000];
    let pointer = 0;

    while counter < str_program.len() {
        if program[counter] == '.' {
            print_memory(memory[pointer]);
        } else if program[counter] == '+' {
            memory[pointer] += 1;
        } else if program[counter] == '-' {
            memory[pointer] -= 1;
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
}
