pub struct Machine {
    memory: Vec<u8>,
    pointer: usize,
}

impl Machine {
    pub fn with_memory_size(size: usize) -> Self {
        Machine {
            memory: vec![0; size],
            pointer: 0,
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
}
