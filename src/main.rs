fn main() {
    let str_program = ".";

    let program: Vec<char> = str_program.chars().collect();
    let counter = 0;

    let memory: Vec<u32> = vec![0; 30000];
    let pointer = 0;

    if program[counter] == '.' {
        print_memory(memory[pointer]);
    }

    print!("\n");
}

fn print_memory(mem: u32) {
    unsafe {
        let c: char = char::from_u32_unchecked(mem);
        print!("{}", c);
    }
}
