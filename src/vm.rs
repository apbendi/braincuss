pub struct Machine {
    memory: Vec<u8>,
    pointer: usize,
    program: Vec<char>,
    pc: usize,
}

impl Machine {
    pub fn with_memory_size_and_program(size: usize, program: &String) -> Self {
        Machine {
            memory: vec![0; size],
            pointer: 0,
            program: program.chars().collect(),
            pc: 0,
        }
    }
    pub fn read_memory(&self) -> u8 {
        self.memory[self.pointer]
    }

    pub fn write_memory(&mut self, c: u8) {
        self.memory[self.pointer] = c;
    }

    pub fn increment_memory(&mut self) {
        if self.memory[self.pointer] == 255 {
            self.memory[self.pointer] = 0;
        } else {
            self.memory[self.pointer] += 1;
        }
    }

    pub fn decrement_memory(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.memory[self.pointer] = 255
        } else {
            self.memory[self.pointer] -= 1;
        }
    }

    pub fn advance_pointer(&mut self) {
        self.pointer += 1;
        if self.pointer == self.memory.len() {
            self.pointer = 0;
        }
    }

    pub fn reverse_pointer(&mut self) {
        if self.pointer == 0 {
            self.pointer = self.memory.len() - 1;
        } else {
            self.pointer -= 1;
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }

    pub fn jump(&mut self, destination: usize) {
        self.pc = destination;
    }

    pub fn instruction(&self) -> char {
        self.program[self.pc]
    }

    pub fn end_of_program(&self) -> bool {
        self.pc >= self.program.len()
    }
}
